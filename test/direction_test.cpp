// direction_test.cpp

#include "../src/model/direction.hpp"

#include <boost/test/unit_test.hpp>

#include <array>


BOOST_AUTO_TEST_SUITE(direction)

using random3dmaze::model::direction;

BOOST_AUTO_TEST_CASE(test_turn_left)
{
  constexpr std::array<direction, 4> expected_list = {{
    direction::west(),
    direction::south(),
    direction::east(),
    direction::north(),
  }};

  direction actual = direction::north();
  for (auto const expected : expected_list)
  {
    actual = actual.turn_left();
    BOOST_CHECK_EQUAL(expected, actual);
  }
}

BOOST_AUTO_TEST_CASE(test_turn_right)
{
  constexpr std::array<direction, 4> expected_list = {{
    direction::east(),
    direction::south(),
    direction::west(),
    direction::north(),
  }};

  direction actual = direction::north();
  for (auto const expected : expected_list)
  {
    actual = actual.turn_right();
    BOOST_CHECK_EQUAL(expected, actual);
  }
}

BOOST_AUTO_TEST_CASE(test_turn_back)
{
  BOOST_CHECK_EQUAL(direction::south(), direction::north().turn_back());
  BOOST_CHECK_EQUAL(direction::north(), direction::south().turn_back());
  BOOST_CHECK_EQUAL(direction::east(), direction::west().turn_back());
  BOOST_CHECK_EQUAL(direction::west(), direction::east().turn_back());
}


using random3dmaze::model::point;

BOOST_AUTO_TEST_CASE(test_direction_of_north)
{
  point const p = point::from_xy(3, 8);

  BOOST_CHECK_EQUAL(point::from_xy(3, 7), direction::north().front_of(p));
  BOOST_CHECK_EQUAL(point::from_xy(3, 9), direction::north().back_of(p));
  BOOST_CHECK_EQUAL(point::from_xy(2, 8), direction::north().left_of(p));
  BOOST_CHECK_EQUAL(point::from_xy(4, 8), direction::north().right_of(p));
}

BOOST_AUTO_TEST_SUITE_END()
