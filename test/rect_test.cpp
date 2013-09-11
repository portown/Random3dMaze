// rect_test.cpp

#include "../src/model/rect.hpp"

#include <boost/test/unit_test.hpp>


BOOST_AUTO_TEST_SUITE(rect)

using random3dmaze::model::rect;
using random3dmaze::model::point;

BOOST_AUTO_TEST_CASE(test_contains)
{
  rect const r = rect::from_topleft_bottomright(
      point::from_xy(0, 0), point::from_xy(10, 10));

  BOOST_CHECK(r.contains(point::from_xy(0, 0)));
  BOOST_CHECK(!r.contains(point::from_xy(-1, -1)));

  BOOST_CHECK(r.contains(point::from_xy(10, 10)));
  BOOST_CHECK(!r.contains(point::from_xy(11, 11)));
}

BOOST_AUTO_TEST_SUITE_END()
