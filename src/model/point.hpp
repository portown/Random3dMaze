// point.hpp

#pragma once

#include <functional>
#include <ostream>


namespace random3dmaze
{
  namespace model
  {
    class point
    {
    public:
      static constexpr auto from_xy(int x, int y) -> point
      {
        return {x, y};
      }

      constexpr auto x() const -> int { return x_; }
      constexpr auto y() const -> int { return y_; }

    private:
      constexpr point(int x, int y) : x_(x), y_(y) {}

      int x_;
      int y_;
    };

    inline constexpr auto operator==(point const& lhs, point const& rhs) -> bool
    {
      return lhs.x() == rhs.x()
        && lhs.y() == rhs.y()
        ;
    }

    inline constexpr auto operator!=(point const& lhs, point const& rhs) -> bool
    {
      return !(lhs == rhs);
    }


    inline auto operator<<(std::ostream& os, point const& p) -> std::ostream&
    {
      os << "{" << p.x() << ", " << p.y() << "}";
      return os;
    }
  }
}

namespace std
{
  template <>
  class less<random3dmaze::model::point>
  {
    auto operator()(
        random3dmaze::model::point const& lhs,
        random3dmaze::model::point const& rhs)
      -> bool
    {
      return lhs.y() < rhs.y()
        || lhs.x() < rhs.x()
        ;
    }
  };
}
