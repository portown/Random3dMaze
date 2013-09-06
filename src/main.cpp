// main.cpp

#define MAIN_DECLARE

// 結合
#include "common.h"

#include <tchar.h>

#ifdef __MINGW32__
#undef MAKEINTRESOURCE
#define MAKEINTRESOURCE(i) ((LPTSTR)(ULONG_PTR)(i))
#endif

// 定数の定義
#define CLSNAME _T("3DMAZE")
#define WNDNAME _T("3D迷路")

#define ID_FORW 100
#define ID_LEFT 101
#define ID_RIGHT 102
#define ID_BACK 103

// 関数の宣言
LRESULT CALLBACK WndProc( HWND, UINT, WPARAM, LPARAM );
BOOL InitApp( HINSTANCE );
BOOL InitInstance( HINSTANCE, int );
int Run( void );


// ==============================================
// 基礎部分
// ==============================================

// エントリポイント
int WINAPI WinMain( HINSTANCE hCurInst, HINSTANCE, LPSTR, int nCmd )
{
  CreateMutex( NULL, FALSE, CLSNAME );

  if ( !CngCurDir() ) return 0;

  if ( !InitApp( hCurInst ) ) return 0;
  if ( !InitInstance( hCurInst, nCmd ) ) return 0;
  if ( !InitGame() ) return 0;

  return Run();
}

// ウィンドウクラスの登録
BOOL InitApp( HINSTANCE hInst )
{
  WNDCLASSEX wc;

  wc.cbSize = sizeof( WNDCLASSEX );
  wc.cbClsExtra = 0;
  wc.cbWndExtra = 0;
  wc.hbrBackground = ( HBRUSH )GetStockObject( WHITE_BRUSH );
  wc.hCursor = ( HCURSOR )LoadImage( NULL, MAKEINTRESOURCE( IDC_ARROW ),
    IMAGE_CURSOR, 0, 0, LR_DEFAULTSIZE | LR_SHARED );
  wc.hIcon = ( HICON )LoadImage( hInst, MAKEINTRESOURCE( IDI_MAIN ),
    IMAGE_ICON, 0, 0, LR_DEFAULTSIZE | LR_SHARED );
  wc.hIconSm = ( HICON )LoadImage( hInst, MAKEINTRESOURCE( IDI_MAIN ),
    IMAGE_ICON, 0, 0, LR_DEFAULTSIZE | LR_SHARED );
  wc.hInstance = hInst;
  wc.lpfnWndProc = ( WNDPROC )WndProc;
  wc.lpszClassName = CLSNAME;
  wc.lpszMenuName = NULL;
  wc.style = CS_HREDRAW | CS_VREDRAW;

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
    NULL,
    NULL,
    hInst,
    NULL );

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

  while ( GetMessage( &msg, NULL, 0, 0L ) > 0 )
  {
    TranslateMessage( &msg );
    DispatchMessage( &msg );
  }

  return ( int )msg.wParam;
}

// ウィンドウプロシージャ
LRESULT CALLBACK WndProc( HWND hWnd, UINT msg, WPARAM wp, LPARAM lp )
{
  //static HWND hForw, hLeft, hRight, hBack;
  PAINTSTRUCT ps;
  //HINSTANCE hInst;
  HDC hDC;
  int tmp;

  switch ( msg )
  {
    case WM_CREATE:
      /*hInst = ( ( LPCREATESTRUCT )lp )->hInstance;
      hForw = CreateWindowEx( 0, _T("BUTTON"), _T("前進"), WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
        256 + 48 * 2 + ( 256 - 80 ) / 2, 48, 80, 24, hWnd, ( HMENU )ID_FORW, hInst, NULL );
      hLeft = CreateWindowEx( 0, _T("BUTTON"), _T("左を向く"), WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
        256 + 48 * 2, 48 + ( 256 - 24 ) / 2, 80, 24, hWnd, ( HMENU )ID_LEFT, hInst, NULL );
      hRight = CreateWindowEx( 0, _T("BUTTON"), _T("右を向く"), WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
        256 * 2 + 48 * 2 - 80, 48 + ( 256 - 24 ) / 2, 80, 24, hWnd, ( HMENU )ID_RIGHT, hInst, NULL );
      hBack = CreateWindowEx( 0, _T("BUTTON"), _T("反転"), WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
        256 + 48 * 2 + ( 256 - 80 ) / 2, 48 + 256 - 24, 80, 24, hWnd, ( HMENU )ID_BACK, hInst, NULL );*/
      break;

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
          InvalidateRect( hWnd, NULL, TRUE );
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
      InvalidateRect( hWnd, NULL, FALSE );
      break;

    /*case WM_COMMAND:
      switch ( LOWORD( wp ) )
      {
        case ID_FORW:
          PostMessage( hWnd, WM_KEYDOWN, VK_UP, 0 );
          break;

        case ID_LEFT:
          PostMessage( hWnd, WM_KEYDOWN, VK_LEFT, 0 );
          break;

        case ID_RIGHT:
          PostMessage( hWnd, WM_KEYDOWN, VK_RIGHT, 0 );
          break;

        case ID_BACK:
          PostMessage( hWnd, WM_KEYDOWN, VK_DOWN, 0 );
          break;

        default:
          return DefWindowProc( hWnd, msg, wp, lp );
      }
      break;*/

    case WM_DESTROY:
      delete [] map;
      PostQuitMessage( 0 );
      break;

    default:
      return DefWindowProc( hWnd, msg, wp, lp );
  }

  return 0L;
}


// EOF


