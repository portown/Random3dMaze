// rect.hpp

#pragma once

#include "point.hpp"

#include <type_traits>
#include <ostream>


namespace random3dmaze
{
  namespace model
  {
    class rect
    {
      using elem_t = decltype(std::declval<point>().x());
      using diff_t = decltype(std::declval<elem_t>() - std::declval<elem_t>());

    public:
      static constexpr auto from_topleft_bottomright(
          point const& topleft,
          point const& bottomright)
        -> rect
      {
        return rect{topleft, bottomright};
      }

      constexpr auto left() const -> elem_t { return topleft_.x(); }
      constexpr auto top() const -> elem_t { return topleft_.y(); }
      constexpr auto right() const -> elem_t { return bottomright_.x(); }
      constexpr auto bottom() const -> elem_t { return bottomright_.y(); }

      constexpr auto width() const -> diff_t { return right() - left() + 1; }
      constexpr auto height() const -> diff_t { return bottom() - top() + 1; }

      constexpr auto topleft() const -> point const& { return topleft_; }
      constexpr auto bottomright() const -> point const& { return bottomright_; }

      constexpr auto topright() const -> point
      {
        return point::from_xy(right(), top());
      }

      constexpr auto bottomleft() const -> point
      {
        return point::from_xy(left(), bottom());
      }

      constexpr auto contains(point const& p) const -> bool
      {
        return p.x() >= left()
            && p.y() >= top()
            && p.x() <= right()
            && p.y() <= bottom()
            ;
      }

    private:
      constexpr explicit rect(point const& topleft, point const& bottomright)
        : topleft_(topleft), bottomright_(bottomright)
      {
      }

      point topleft_;
      point bottomright_;
    };

    inline auto operator<<(std::ostream& os, rect const& r) -> std::ostream&
    {
      os << "{{" << r.left() << ", " << r.top() << "}, {"
         << r.right() << ", " << r.bottom() << "}}";

      return os;
    }
  }
}
