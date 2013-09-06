// Tips.cpp

#include <time.h>
#include <windows.h>
#include <tchar.h>

#include "resource.h"
#include "structs.h"
#include "funcs.h"
#include "vars.h"


// 簡易メッセージボックス
int Mes( LPCTSTR lpText, LPCTSTR lpCaption, UINT uType, HWND hWnd )
{
  return MessageBox( hWnd, lpText, lpCaption, uType );
}

// バッファの初期化
void InitSurface( HDC &hDC, HBITMAP &hBm, int w, int h )
{
  HDC const hTemp = GetDC( ghWnd );

  BITMAPINFO bi;
  bi.bmiHeader.biSize = sizeof( BITMAPINFOHEADER );
  bi.bmiHeader.biWidth = w;
  bi.bmiHeader.biHeight = -h;
  bi.bmiHeader.biPlanes = 1;
  bi.bmiHeader.biBitCount = 32;
  bi.bmiHeader.biCompression = BI_RGB;

  hDC = CreateCompatibleDC( hTemp );
  hBm = CreateDIBSection( hTemp, &bi, DIB_RGB_COLORS, nullptr, nullptr, 0 );
  if ( hBm == nullptr )
  {
    Mes( _T("サーフェイスを正しく作成できませんでした") );
    return;
  }
  SelectObject( hDC, hBm );

  ReleaseDC( ghWnd, hTemp );

  RECT rc;
  rc.left = rc.top = 0;
  rc.right = w;
  rc.bottom = h;
  PaintRect( hDC, rc, RGB( 0, 0, 0 ) );
}

// デバイスコンテキスト・ビットマップハンドルの解放
void RelsSurface( HDC &hDC, HBITMAP &hBm )
{
  if ( hBm )
  {
    DeleteObject( hBm );
    hBm = nullptr;
  }
  if ( hDC )
  {
    DeleteDC( hDC );
    hDC = nullptr;
  }
}

// 矩形塗りつぶし
void PaintRect( HDC hDC, RECT rc, COLORREF col )
{
  HBRUSH hBrush, hOld;

  hBrush = CreateSolidBrush( col );

  hOld = ( HBRUSH )SelectObject( hDC, hBrush );
  PatBlt( hDC, rc.left, rc.top, rc.right - rc.left, rc.bottom - rc.top, PATCOPY );
  SelectObject( hDC, hOld );

  DeleteObject( hBrush );
}

// 簡易矩形作成
RECT MakeRect( int x, int y, int w, int h )
{
  RECT rc;

  SetRect( &rc, x, y, w + x, h + y );

  return rc;
}

// 画像の読み込み
int Load_Bmp( HDC hDC, const TCHAR *f_name )
{
  HBITMAP const bitmap = static_cast<HBITMAP>(LoadImage(nullptr, f_name, IMAGE_BITMAP, 0, 0, LR_CREATEDIBSECTION | LR_LOADFROMFILE));
  if (!bitmap)
  {
    TCHAR szErr[256];
    wsprintf( szErr, _T("【%s】：ビットマップファイルの読み込みに失敗しました"), f_name );
    Mes( szErr );
    return -1;
  }

  BITMAP bitmapInfo;
  GetObject(bitmap, sizeof(bitmapInfo), &bitmapInfo);

  HDC const tempDC = CreateCompatibleDC(hDC);
  SelectObject(tempDC, bitmap);

  BitBlt(hDC, 0, 0, bitmapInfo.bmWidth, bitmapInfo.bmHeight, tempDC, 0, 0, SRCCOPY);

  DeleteDC(tempDC);
  DeleteObject(bitmap);

  return 0;
}
