// main.cpp

#include <time.h>
#include <windows.h>
#include <tchar.h>

#include "resource.h"
#include "structs.h"
#include "funcs.h"

#define MAIN_DECLARE
#include "vars.h"
#undef MAIN_DECLARE

#ifdef __MINGW32__
#undef MAKEINTRESOURCE
#define MAKEINTRESOURCE(i) ((LPTSTR)(ULONG_PTR)(i))
#endif


namespace
{
  constexpr auto CLSNAME = _T("3DMAZE");
  constexpr auto WNDNAME = _T("3D迷路");

  LRESULT CALLBACK WndProc( HWND, UINT, WPARAM, LPARAM );
  BOOL InitApp( HINSTANCE );
  BOOL InitInstance( HINSTANCE, int );
  int Run();
}


auto WINAPI WinMain( HINSTANCE hCurInst, HINSTANCE, LPSTR, int nCmd ) -> int
{
  CreateMutex( nullptr, FALSE, CLSNAME );

  if ( !InitApp( hCurInst ) ) return 0;
  if ( !InitInstance( hCurInst, nCmd ) ) return 0;
  if ( !InitGame() ) return 0;

  return Run();
}


namespace
{
  // ウィンドウクラスの登録
  BOOL InitApp( HINSTANCE hInst )
  {
    WNDCLASSEX wc;

    wc.cbSize = sizeof(wc);
    wc.cbClsExtra = 0;
    wc.cbWndExtra = 0;
    wc.hbrBackground = ( HBRUSH )GetStockObject( WHITE_BRUSH );
    wc.hCursor = ( HCURSOR )LoadImage( nullptr, MAKEINTRESOURCE( IDC_ARROW ),
        IMAGE_CURSOR, 0, 0, LR_DEFAULTSIZE | LR_SHARED );
    wc.hIcon = ( HICON )LoadImage( hInst, MAKEINTRESOURCE( IDI_MAIN ),
        IMAGE_ICON, 0, 0, LR_DEFAULTSIZE | LR_SHARED );
    wc.hIconSm = ( HICON )LoadImage( hInst, MAKEINTRESOURCE( IDI_MAIN ),
        IMAGE_ICON, 0, 0, LR_DEFAULTSIZE | LR_SHARED );
    wc.hInstance = hInst;
    wc.lpfnWndProc = WndProc;
    wc.lpszClassName = CLSNAME;
    wc.lpszMenuName = nullptr;
    wc.style = 0;

    if ( !RegisterClassEx( &wc ) )
      return FALSE;

    return TRUE;
  }

  // ウィンドウの作成
  BOOL InitInstance( HINSTANCE hInst, int nCmd )
  {
    HWND hWnd;

    hWnd = CreateWindowEx( 0,
        CLSNAME,
        WNDNAME,
        WS_SYSMENU | WS_CAPTION | WS_MINIMIZEBOX,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        48 * 3 + 256 * 2 + GetSystemMetrics( SM_CXDLGFRAME ) * 2,
        48 * 2 + 256 + GetSystemMetrics( SM_CYDLGFRAME ) * 2 + GetSystemMetrics( SM_CYCAPTION ),
        nullptr,
        nullptr,
        hInst,
        nullptr );

    if ( !hWnd ) return FALSE;

    ghWnd = hWnd;

    ShowWindow( hWnd, nCmd );
    UpdateWindow( hWnd );

    return TRUE;
  }

  // メッセージ・ループ
  int Run( void )
  {
    MSG msg;

    while ( GetMessage( &msg, nullptr, 0, 0L ) > 0 )
    {
      TranslateMessage( &msg );
      DispatchMessage( &msg );
    }

    return ( int )msg.wParam;
  }

  // ウィンドウプロシージャ
  LRESULT CALLBACK WndProc( HWND hWnd, UINT msg, WPARAM wp, LPARAM lp )
  {
    PAINTSTRUCT ps;
    HDC hDC;
    int tmp;

    switch ( msg )
    {
      case WM_PAINT:
        hDC = BeginPaint( hWnd, &ps );
        if ( mapw ) Draw( hDC );
        EndPaint( hWnd, &ps );
        break;

      case WM_LBUTTONDOWN:
        SetFocus( hWnd );
        break;

      case WM_KEYDOWN:
        switch ( wp )
        {
          case VK_LEFT:
            tmp = pl.dp.x;
            pl.dp.x = pl.dp.y;
            pl.dp.y = -tmp;
            break;

          case VK_UP:
            if ( map[pl.x + pl.dp.x + ( pl.y + pl.dp.y ) * mapw] == MI_WALL )
              break;

            pl.x += pl.dp.x;
            pl.y += pl.dp.y;
            if ( pl.x < 0 ) pl.x = 0;
            if ( pl.x > ( mapw - 1 ) ) pl.x = mapw - 1;
            if ( pl.y < 0 ) pl.y = 0;
            if ( pl.y > ( maph - 1 ) ) pl.y = maph - 1;
            break;

          case VK_RIGHT:
            tmp = pl.dp.y;
            pl.dp.y = pl.dp.x;
            pl.dp.x = -tmp;
            break;

          case VK_DOWN:
            pl.dp.x = -pl.dp.x;
            pl.dp.y = -pl.dp.y;
            break;

          case 'M':
            bMMap = !bMMap;
            InvalidateRect( hWnd, nullptr, TRUE );
            --nkey;
            ++nmap;
            break;

          case VK_ESCAPE:
            DestroyWindow( hWnd );
            break;

          default:
            return DefWindowProc( hWnd, msg, wp, lp );
        }
        ++nkey;
        DrawWall();
        InvalidateRect( hWnd, nullptr, FALSE );
        break;

      case WM_DESTROY:
        delete [] map;
        PostQuitMessage( 0 );
        break;

      default:
        return DefWindowProc( hWnd, msg, wp, lp );
    }

    return 0L;
  }
}
