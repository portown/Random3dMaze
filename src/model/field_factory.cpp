// field_factory.cpp

#include "field_factory.hpp"

#include <utility>

#include "direction.hpp"

namespace ns = random3dmaze::model;


auto ns::field_factory::create() const -> field
{
  rect const area = rect::from_topleft_bottomright(
      point::from_xy(0, 0), point::from_xy(18, 18)
  );

  std::vector<bool> walls(area.width() * area.height(), false);
  for (auto p = area.topleft(); p != area.topright(); p = direction::east().front_of(p))
  {
    walls[p.x()] = true;
  }
  for (auto p = area.topright(); p != area.bottomright(); p = direction::south().front_of(p))
  {
    walls[p.x() + p.y() * area.width()] = true;
  }
  walls.back() = true;

  return field{std::move(walls), area};
}
