// field_factory.hpp

#pragma once

#include "field.hpp"


namespace random3dmaze
{
  namespace model
  {
    class field_factory
    {
    public:
      auto create() const -> field;
    };
  }
}
