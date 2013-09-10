// direction.hpp

#pragma once

#include "point.hpp"

#include <ostream>


namespace random3dmaze
{
  namespace model
  {
    class direction
    {
    public:
      static constexpr auto north() -> direction { return {0, -1}; }
      static constexpr auto south() -> direction { return {0, 1}; }
      static constexpr auto west() -> direction { return {-1, 0}; }
      static constexpr auto east() -> direction { return {1, 0}; }

      constexpr auto turn_left() const -> direction { return {latitude_, -longitude_}; }
      constexpr auto turn_right() const -> direction { return {-latitude_, longitude_}; }
      constexpr auto turn_back() const -> direction { return {-longitude_, -latitude_}; }

      constexpr auto front_of(point const& p) const -> point
      {
        return point::from_xy(p.x() + longitude_, p.y() + latitude_);
      }

      constexpr auto back_of(point const& p) const -> point
      {
        return turn_back().front_of(p);
      }

      constexpr auto left_of(point const& p) const -> point
      {
        return turn_left().front_of(p);
      }

      constexpr auto right_of(point const& p) const -> point
      {
        return turn_right().front_of(p);
      }

      friend constexpr auto operator==(direction const&, direction const&) -> bool;

    private:
      constexpr direction(int longitude, int latitude)
        : longitude_(longitude), latitude_(latitude)
      {
      }

      int longitude_;
      int latitude_;
    };

    constexpr auto operator==(direction const& lhs, direction const& rhs) -> bool
    {
      return lhs.longitude_ == rhs.longitude_
        && lhs.latitude_ == rhs.latitude_
        ;
    }

    constexpr auto operator!=(direction const& lhs, direction const& rhs) -> bool
    {
      return !(lhs == rhs);
    }

    auto operator<<(std::ostream& os, direction const& direction) -> std::ostream&
    {
      if (direction == direction::north()) os << "north";
      else if (direction == direction::south()) os << "south";
      else if (direction == direction::west()) os << "west";
      else if (direction == direction::east()) os << "east";
      else os << "unknown direction";

      return os;
    }
  }
}
