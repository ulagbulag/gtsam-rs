#include <boost/smart_ptr/shared_ptr.hpp>
#include <memory>

namespace {
template <class SharedPointer> struct Holder {
  SharedPointer p;

  Holder(const SharedPointer &p) : p(p) {}
  Holder(const Holder &other) : p(other.p) {}
  Holder(Holder &&other) : p(std::move(other.p)) {}

  void operator()(...) { p.reset(); }
};
} // namespace

template <class T>
std::shared_ptr<T> to_std_ptr(const boost::shared_ptr<T> &p) {
  typedef Holder<std::shared_ptr<T>> H;
  if (p == nullptr) {
    return nullptr;
  } else if (H *h = boost::get_deleter<H>(p)) {
    return h->p;
  } else {
    return std::shared_ptr<T>(p.get(), Holder<boost::shared_ptr<T>>(p));
  }
}

template <class T>
boost::shared_ptr<T> to_boost_ptr(const std::shared_ptr<T> &p) {
  typedef Holder<boost::shared_ptr<T>> H;
  if (p == nullptr) {
    return nullptr;
  } else if (H *h = std::get_deleter<H>(p)) {
    return h->p;
  } else {
    return boost::shared_ptr<T>(p.get(), Holder<std::shared_ptr<T>>(p));
  }
}

template <class Child, class Parent>
std::shared_ptr<Parent> downcast(const std::shared_ptr<Child> &p) {
  boost::shared_ptr<Child> p_boost = to_boost_ptr(p);
  boost::shared_ptr<Parent> p_casted = p_boost;
  return to_std_ptr(p_casted);
}
