// Tips.cpp

#include <time.h>
#include <windows.h>
#include <tchar.h>

#include "resource.h"
#include "structs.h"
#include "funcs.h"
#include "vars.h"

// 関数のプロトタイプ宣言
HPALETTE SetPalette( LPBITMAPINFOHEADER );
int MakeBmpInfo( LPBITMAPINFOHEADER, LPBITMAPINFO, char * );


// ==============================================
// 頻出関数群
// ==============================================

// 簡易メッセージボックス
int Mes( LPCTSTR lpText, LPCTSTR lpCaption, UINT uType, HWND hWnd )
{
  return MessageBox( hWnd, lpText, lpCaption, uType );
}

// バッファの初期化
void InitSurface( HDC &hDC, HBITMAP &hBm, int w, int h )
{
  BITMAPINFO bi;
  RECT rc;
  HDC hTemp;

  hTemp = GetDC( ghWnd );

  bi.bmiHeader.biSize = sizeof( BITMAPINFOHEADER );
  bi.bmiHeader.biWidth = w;
  bi.bmiHeader.biHeight = -h;
  bi.bmiHeader.biPlanes = 1;
  bi.bmiHeader.biBitCount = 32;
  bi.bmiHeader.biCompression = BI_RGB;

  hDC = CreateCompatibleDC( hTemp );
  hBm = CreateDIBSection( hTemp, &bi, DIB_RGB_COLORS, NULL, NULL, 0 );
  if ( hBm == NULL )
  {
    Mes( _T("サーフェイスを正しく作成できませんでした") );
    return;
  }
  SelectObject( hDC, hBm );

  ReleaseDC( ghWnd, hTemp );

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
    hBm = NULL;
  }
  if ( hDC )
  {
    DeleteDC( hDC );
    hDC = NULL;
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
  HANDLE hFile;
  HANDLE hMemFile, hMemIfHd, hMemInfo, hMemBuff;
  LPBITMAPFILEHEADER lpBf;
  LPBITMAPINFOHEADER lpBi;
  LPBITMAPINFO lpBmpInfo;
  char *lpBuffer;
  HPALETTE hPalette;
  DWORD dwRead;
  TCHAR szErr[0x100];
  char szType[3];
  int i;

  hFile = CreateFile( f_name, GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING,
    FILE_ATTRIBUTE_NORMAL, NULL );
  if ( hFile == INVALID_HANDLE_VALUE )
  {
    wsprintf( szErr, _T("【%s】：ビットマップファイルの読み込みに失敗しました"), f_name );
    Mes( szErr );
    return -1;
  }

  hMemFile = GlobalAlloc( GHND, sizeof( BITMAPFILEHEADER ) );
  lpBf = ( LPBITMAPFILEHEADER )GlobalLock( hMemFile );

  SetFilePointer( hFile, 0, 0, FILE_BEGIN );
  ReadFile( hFile, ( LPVOID )lpBf, sizeof( BITMAPFILEHEADER ), &dwRead, NULL );

  szType[0] = LOBYTE( lpBf->bfType );
  szType[1] = HIBYTE( lpBf->bfType );
  szType[2] = 0x00;
  if ( strcmp( szType, "BM" ) )
  {
    Mes( _T("ビットマップファイルではありません") );
    GlobalUnlock( hMemFile );
    GlobalFree( hMemFile );
    CloseHandle( hFile );
    return -2;
  }

  hMemIfHd = GlobalAlloc( GHND, sizeof( BITMAPINFOHEADER ) );
  lpBi = ( LPBITMAPINFOHEADER )GlobalLock( hMemIfHd );

  ReadFile( hFile, ( LPVOID )lpBi, sizeof( BITMAPINFOHEADER ), &dwRead, NULL );

  if ( lpBi->biBitCount < 16 && lpBi->biClrUsed == 0 )
  {
    lpBi->biClrUsed = 1;

    for ( i = 0; i < lpBi->biBitCount; i++ )
    {
      lpBi->biClrUsed *= 2;
    }
  }

  hMemInfo = GlobalAlloc( GHND,
    sizeof( BITMAPINFO ) + lpBi->biClrUsed * sizeof( RGBQUAD ) );
  lpBmpInfo = ( LPBITMAPINFO )GlobalLock( hMemInfo );

  hMemBuff = GlobalAlloc( GHND, lpBf->bfSize - sizeof( BITMAPFILEHEADER ) );
  lpBuffer = ( char * )GlobalLock( hMemBuff );

  SetFilePointer( hFile, sizeof( BITMAPFILEHEADER ), 0, FILE_BEGIN );
  ReadFile( hFile, ( LPVOID )lpBuffer,
    lpBf->bfSize - lpBf->bfOffBits + lpBi->biClrUsed *
    sizeof( RGBQUAD ) + sizeof( BITMAPINFOHEADER ),
    &dwRead, NULL );

  if ( lpBi->biClrUsed != 0 || lpBi->biBitCount != 24 )
    hPalette = SetPalette( lpBi );

  lpBmpInfo->bmiHeader = *lpBi;
  MakeBmpInfo( lpBi, lpBmpInfo, lpBuffer );

  SetDIBitsToDevice( hDC,
    0, 0, lpBi->biWidth, lpBi->biHeight,
    0, 0, 0, lpBi->biHeight,
    ( char * )lpBuffer + lpBf->bfOffBits - sizeof( BITMAPFILEHEADER ),
    lpBmpInfo,
    DIB_RGB_COLORS );

  GlobalUnlock( hMemBuff );
  GlobalFree( hMemBuff );

  GlobalUnlock( hMemInfo );
  GlobalFree( hMemInfo );

  GlobalUnlock( hMemIfHd );
  GlobalFree( hMemIfHd );

  GlobalUnlock( hMemFile );
  GlobalFree( hMemFile );

  DeleteObject( hPalette );

  CloseHandle( hFile );

  return 0;
}

// パレットの設定
HPALETTE SetPalette( LPBITMAPINFOHEADER lpBi )
{
  LPLOGPALETTE lpPal;
  LPRGBQUAD lpRGB;
  HANDLE hPal;
  HPALETTE hPalette;
  WORD i;

  hPal = GlobalAlloc( GHND, sizeof( LOGPALETTE ) +
    lpBi->biClrUsed * sizeof( PALETTEENTRY ) );
  lpPal = ( LPLOGPALETTE )GlobalLock( hPal );

  lpPal->palVersion = 0x300;
  lpPal->palNumEntries = ( WORD )lpBi->biClrUsed;

  lpRGB = ( LPRGBQUAD )( ( LPSTR )lpBi + lpBi->biSize );

  for ( i = 0; i < lpBi->biClrUsed; i++, lpRGB++ )
  {
    lpPal->palPalEntry[i].peRed   = lpRGB->rgbRed;
    lpPal->palPalEntry[i].peGreen = lpRGB->rgbGreen;
    lpPal->palPalEntry[i].peBlue  = lpRGB->rgbBlue;
    lpPal->palPalEntry[i].peFlags = 0;
  }

  GlobalUnlock( hPal );
  hPalette = CreatePalette( lpPal );
  if ( !hPalette )
  {
    Mes( _T("パレットの作成に失敗しました") );
  }

  GlobalFree( hPal );

  return hPalette;
}

// BITMAPINFOのデータ入力
int MakeBmpInfo( LPBITMAPINFOHEADER lpBi, LPBITMAPINFO lpBmpInfo, char *lpBuffer )
{
  LPRGBQUAD lpRGB;
  int i;

  lpRGB = ( LPRGBQUAD )( lpBuffer + sizeof( BITMAPINFOHEADER ) );

  for ( i = 0; i < ( int )lpBi->biClrUsed; i++ )
  {
    lpBmpInfo->bmiColors[i].rgbRed    = lpRGB->rgbRed;
    lpBmpInfo->bmiColors[i].rgbGreen  = lpRGB->rgbGreen;
    lpBmpInfo->bmiColors[i].rgbBlue   = lpRGB->rgbBlue;
    lpBmpInfo->bmiColors[i].rgbReserved = 0;
    lpRGB++;
  }

  return 1;
}


// EOF
