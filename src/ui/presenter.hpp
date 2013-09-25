// presenter.hpp

#pragma once

#include <Windows.h>


namespace random3dmaze
{
  namespace ui
  {
    class presenter
    {
    public:
      explicit presenter(HINSTANCE instance_handle);

      auto run() -> int;

    private:
      HINSTANCE instance_handle_;
    };
  }
}
