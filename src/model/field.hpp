// field.hpp

#pragma once

#include <vector>
#include <utility>

#include "point.hpp"
#include "rect.hpp"


namespace random3dmaze
{
  namespace model
  {
    class field_factory;

    class field
    {
      using size_type = decltype(std::declval<rect>().width());

    public:
      friend class field_factory;

      auto width() const -> size_type { return area_.width(); }
      auto height() const -> size_type { return area_.height(); }

      auto point_of_start() const -> point const& { return area_.topleft(); }
      auto point_of_goal() const -> point const& { return area_.bottomright(); }

      auto has_wall_at(point const& p) const -> bool
      {
        if (!area_.contains(p)) return false;

        return walls_[to_index(p)];
      }

    private:
      explicit field(std::vector<bool>&& walls, rect const& area)
        : walls_{std::move(walls)}, area_{area}
      {
      }

      auto to_index(point const& p) const -> std::size_t
      {
        return p.x() + p.y() * width();
      }

      std::vector<bool> walls_;
      rect area_;
    };
  }
}
