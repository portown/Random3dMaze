// main.cpp

#include <Windows.h>

#include "ui/presenter.hpp"


auto WINAPI WinMain(HINSTANCE instance_handle, HINSTANCE, LPSTR, int)
  -> int
{
  return random3dmaze::ui::presenter{instance_handle}.run();
}
