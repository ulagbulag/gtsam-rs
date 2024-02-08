use std::{env, path::PathBuf};

use glob::glob;

#[cfg(unix)]
fn find_include_dirs(name: &str) -> Vec<PathBuf> {
    use std::{process::Command, str::FromStr};

    let output = Command::new("whereis")
        .arg(name)
        .output()
        .expect("failed to execute \"whereis\" command");

    let stdout = String::from_utf8(output.stdout).expect("failed to parse whereis stdout");
    stdout
        .split('\n')
        .map(|line| line.trim())
        .flat_map(|line| line.split(' ').skip(1))
        .filter(|path| path.contains("/include"))
        .filter_map(|path| PathBuf::from_str(path).ok())
        .collect()
}

fn get_target() -> String {
    let triples = env::var("TARGET").expect("failed to get environment variable: TARGET");

    let mut triples: Vec<_> = triples.split('-').collect();
    triples.remove(1);
    triples.join("-")
}

fn get_out_dir() -> PathBuf {
    env::var("OUT_DIR")
        .expect("failed to get environment variable: OUT_DIR")
        .parse()
        .expect("failed to parse environment variable: OUT_DIR")
}

#[cfg(feature = "build")]
struct Repository {
    path: PathBuf,
}

impl Repository {
    // Configure
    const URL: &'static str = concat!(
        "https://github.com/borglab/gtsam/archive/refs/tags/",
        env!("CARGO_PKG_VERSION_MAJOR"),
        ".",
        env!("CARGO_PKG_VERSION_MINOR"),
        ".tar.gz",
    );
    const VERSION: &'static str = concat!(
        env!("CARGO_PKG_VERSION_MAJOR"),
        ".",
        env!("CARGO_PKG_VERSION_MINOR"),
    );
}

#[cfg(feature = "build")]
impl Default for Repository {
    fn default() -> Self {
        // Configure
        let mut path = get_out_dir();

        // Download
        let response =
            ::reqwest::blocking::get(Self::URL).expect("failed to download source codes");

        // Decompress
        {
            let mut archive = ::tar::Archive::new(::flate2::read::GzDecoder::new(response));
            archive
                .unpack(&path)
                .expect("failed to unpack source codes");

            path.push(format!("gtsam-{version}", version = Self::VERSION));
        }
        Self { path }
    }
}

#[cfg(feature = "build")]
impl Repository {
    fn build(&self) -> Library {
        trait CmakeFlag {
            fn to_bool(&self) -> &str;
        }

        impl CmakeFlag for bool {
            fn to_bool(&self) -> &str {
                if *self {
                    "ON"
                } else {
                    "OFF"
                }
            }
        }

        // Configure
        let install_dir = {
            let mut path = self.path.clone();
            path.pop();
            path
        };

        let mut builder = ::cmake::Config::new(&self.path);
        builder
            .define("BUILD_SHARED_LIBS", false.to_bool())
            .define("DEBUG", false.to_bool())
            .define("GDB", false.to_bool())
            .define("GTSAM_BUILD_EXAMPLES_ALWAYS", false.to_bool())
            .define("GTSAM_BUILD_METIS_EXECUTABLES", false.to_bool())
            .define("GTSAM_BUILD_PYTHON", false.to_bool())
            .define("GTSAM_BUILD_TESTS", false.to_bool())
            .define("GTSAM_BUILD_TIMING_ALWAYS", false.to_bool())
            .define("GTSAM_BUILD_TYPE_POSTFIXES", true.to_bool())
            .define("GTSAM_BUILD_WITH_CCACHE", true.to_bool())
            .define("GTSAM_BUILD_WITH_MARCH_NATIVE", false.to_bool())
            .define("GTSAM_CMAKE_BUILD_TYPE", "Release")
            .define("GTSAM_INSTALL_CPPUNITLITE", true.to_bool())
            .define("GTSAM_INSTALL_GEOGRAPHICLIB", false.to_bool())
            .define("GTSAM_INSTALL_MATLAB_TOOLBOX", false.to_bool())
            .define("GTSAM_POSE3_EXPMAP", true.to_bool())
            .define("GTSAM_ROT3_EXPMAP", true.to_bool())
            .define("GTSAM_SUPPORT_NESTED_DISSECTION", true.to_bool())
            .define("GTSAM_TANGENT_PREINTEGRATION", true.to_bool())
            .define("GTSAM_THROW_CHEIRALITY_EXCEPTI", true.to_bool())
            .define("GTSAM_UNSTABLE_BUILD_PYTHON", false.to_bool())
            .define("GTSAM_UNSTABLE_INSTALL_MATLAB_TOOLBOX", false.to_bool())
            .define("GTSAM_USE_QUATERNIONS", false.to_bool())
            .define("GTSAM_WITH_EIGEN_UNSUPPORTED", false.to_bool())
            .define("PCRE", false.to_bool())
            .profile("Release");

        // Assert to use bundled sub-packages
        {
            let use_system_libs = cfg!(feature = "build-use-system-libs").to_bool();

            builder
                .define("GTSAM_BUILD_UNSTABLE", false.to_bool())
                .define("GTSAM_USE_SYSTEM_EIGEN", use_system_libs)
                .define("GTSAM_USE_SYSTEM_METIS", use_system_libs);
        }

        // Configure Accelerators
        {
            let use_mkl = cfg!(feature = "build-use-mkl").to_bool();
            let use_tbb = cfg!(feature = "build-use-tbb").to_bool();

            builder
                .define("GTSAM_WITH_EIGEN_MKL", use_mkl)
                .define("GTSAM_WITH_EIGEN_MKL_OPENMP", use_mkl)
                .define("GTSAM_WITH_TBB", use_mkl)
                .define("OPENMP", use_tbb);
        }

        // Configure Options
        builder.define("GTSAM_SLOW_BUT_CORRECT_BETWEENFACTOR", false.to_bool());

        // Build
        builder.build();

        Library {
            includes: vec![install_dir.join("include")],
            libs: vec![install_dir.join("lib")],
        }
    }
}

struct Library {
    includes: Vec<PathBuf>,
    libs: Vec<PathBuf>,
}

impl Library {
    const EXTERNAL_LIBS: &'static [&'static str] = &["boost_chrono", "boost_timer", "gtsam"];
    const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

    fn find() -> Result<Self, ::pkg_config::Error> {
        let library = ::pkg_config::probe_library("gtsam")?;

        Ok(Self {
            includes: library.include_paths,
            libs: library
                .libs
                .into_iter()
                .filter_map(|path| path.parse::<PathBuf>().ok())
                .collect(),
        })
    }

    fn build(&mut self) {
        fn watch_files(files: &[PathBuf]) {
            files.iter().for_each(|file| {
                println!("cargo:rerun-if-changed={file}", file = file.display());
            })
        }

        let cc_files: Vec<_> = glob("./src/**/*.cc")
            .expect("failed to find C++ source files")
            .filter_map(Result::ok)
            .map(|file| file.to_path_buf())
            .collect();
        let h_files: Vec<_> = glob("./src/**/*.h")
            .expect("failed to find C++ header files")
            .filter_map(Result::ok)
            .map(|file| file.to_path_buf())
            .collect();
        let rs_files: Vec<_> = glob("./src/**/*.rs")
            .expect("failed to find Rust source files")
            .filter_map(Result::ok)
            .map(|file| file.to_path_buf())
            .collect();

        watch_files(&cc_files);
        watch_files(&h_files);
        watch_files(&rs_files);

        ::cxx_build::bridges(rs_files)
            .cpp(true)
            .files(cc_files)
            // NOTE: RAII patterns are available on C++14
            .flag_if_supported("-std=c++14")
            .include(format!("{}/src", env!("CARGO_MANIFEST_DIR")))
            .includes(&self.includes)
            .includes(find_include_dirs("boost"))
            .includes(find_include_dirs("eigen3"))
            .compile(Self::PKG_NAME);

        self.libs.push(get_out_dir())
    }

    fn link(&self) {
        // Link external libraries
        for lib in Self::EXTERNAL_LIBS {
            println!("cargo:rustc-link-lib=static={lib}");
        }

        // Link target-dependent libraries
        // NOTE: cross-target building may requires specific target-dependent libraries (i.e. boost)
        #[cfg(unix)]
        {
            let path = format!("/usr/lib/{}", get_target());
            println!("cargo:rustc-flags=-L {path}");
            println!("cargo:rustc-link-search={path}");
        }

        // Include & Link this packge
        for path in &self.includes {
            println!("cargo:include={}", path.display());
        }
        for path in &self.libs {
            println!("cargo:lib={}", path.display());
            println!("cargo:rustc-flags=-L {}", path.display());
            println!("cargo:rustc-link-search={}", path.display());
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=./build.rs");
    println!("cargo:rerun-if-changed=./Cargo.toml");

    let mut library = match Library::find() {
        Ok(library) => library,
        #[cfg(feature = "build")]
        Err(_) => {
            let repo = Repository::default();
            repo.build()
        }
        #[cfg(not(feature = "build"))]
        Err(error) => panic!("failed to find \"gtsam\" library; you can enable \"build\" feature to skip the step: {error}"),
    };

    library.build();
    library.link()
}
