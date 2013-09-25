// presenter.cpp

#include "presenter.hpp"

#include <boost/optional.hpp>
#include <boost/utility/string_ref.hpp>

#include "../resource.h"

namespace ns = random3dmaze::ui;


namespace
{
  using tstring_ref = boost::basic_string_ref<TCHAR>;

  auto create_window(
      HINSTANCE const instance_handle,
      tstring_ref const& class_name,
      tstring_ref const& window_title,
      int width, int height,
      ns::presenter& owner) -> boost::optional<HWND>;
  auto message_loop() -> int;
  auto CALLBACK window_procedure(
      HWND window,
      UINT message,
      WPARAM param1,
      LPARAM param2) -> LRESULT;
}


ns::presenter::presenter(HINSTANCE const instance_handle)
  : instance_handle_(instance_handle)
{
}

auto ns::presenter::run() -> int
{
  constexpr int width = 48 * 3 + 256 * 2;
  constexpr int height = 48 * 2 + 256;

  auto const window = create_window(instance_handle_,
      TEXT("Random3dMaze"), TEXT("3D迷路"), width, height, *this);
  if (!window) return 0;

  ShowWindow(*window, SW_SHOW);
  UpdateWindow(*window);

  return message_loop();
}


namespace
{
  auto create_window(
      HINSTANCE const instance_handle,
      tstring_ref const& class_name,
      tstring_ref const& window_title,
      int const width,
      int const height,
      ns::presenter& owner)
    -> boost::optional<HWND>
  {
    WNDCLASSEX wc;
    ZeroMemory(&wc, sizeof(wc));
    wc.cbSize = sizeof(wc);
    wc.cbClsExtra = 0;
    wc.cbWndExtra = 0;
    wc.hbrBackground = static_cast<HBRUSH>(GetStockObject(WHITE_BRUSH));
    wc.hCursor = static_cast<HCURSOR>(LoadImage(nullptr, IDC_ARROW,
          IMAGE_CURSOR, 0, 0, LR_DEFAULTSIZE | LR_SHARED));
    wc.hIcon = static_cast<HICON>(LoadImage(instance_handle,
          MAKEINTRESOURCE(IDI_MAIN), IMAGE_ICON, 0, 0, LR_DEFAULTSIZE | LR_SHARED));
    wc.hIconSm = static_cast<HICON>(LoadImage(instance_handle,
          MAKEINTRESOURCE(IDI_MAIN), IMAGE_ICON, 0, 0, LR_DEFAULTSIZE | LR_SHARED));
    wc.hInstance = instance_handle;
    wc.lpfnWndProc = window_procedure;
    wc.lpszClassName = class_name.data();
    wc.lpszMenuName = nullptr;
    wc.style = 0;

    if (!RegisterClassEx(&wc)) return boost::none;

    int const window_width = width + GetSystemMetrics(SM_CXFIXEDFRAME) * 2;
    int const window_height = height + GetSystemMetrics(SM_CYFIXEDFRAME) * 2
      + GetSystemMetrics(SM_CYCAPTION);
    int const central_window_left = (GetSystemMetrics(SM_CXSCREEN) - window_width) / 2;
    int const central_window_top = (GetSystemMetrics(SM_CYSCREEN) - window_height) / 2;

    HWND const window = CreateWindowEx(0,
        class_name.data(),
        window_title.data(),
        WS_SYSMENU | WS_CAPTION | WS_MINIMIZEBOX,
        central_window_left,
        central_window_top,
        window_width,
        window_height,
        nullptr,
        nullptr,
        instance_handle,
        &owner);
    if (!window) return boost::none;

    return boost::make_optional(window);
  }

  auto message_loop() -> int
  {
    MSG message;
    while (BOOL result = GetMessage(&message, nullptr, 0, 0))
    {
      if (result == -1) break;

      TranslateMessage(&message);
      DispatchMessage(&message);
    }

    return static_cast<int>(message.wParam);
  }

  auto CALLBACK window_procedure(
      HWND const window,
      UINT const message,
      WPARAM const param1,
      LPARAM const param2)
    -> LRESULT
  {
    static ns::presenter* owner;

    switch (message)
    {
      case WM_CREATE:
        owner = static_cast<ns::presenter*>(
            reinterpret_cast<LPCREATESTRUCT>(param2)->lpCreateParams);
        static_cast<void>(owner);
        break;

      case WM_DESTROY:
        PostQuitMessage(0);
        break;

      default:
        return DefWindowProc(window, message, param1, param2);
    }

    return 0;
  }
}
