// funcs.h

#ifndef FUNCTIONS_HEADER
#define FUNCTIONS_HEADER

#include <tchar.h>


// ==============================================
// 関数の宣言
// ==============================================

// Tips.cpp
int Mes( LPCTSTR = NULL, LPCTSTR = NULL, UINT = MB_OK, HWND = NULL );
void InitSurface( HDC &, HBITMAP &, int, int );
void RelsSurface( HDC &, HBITMAP & );
void PaintRect( HDC, RECT, COLORREF );
RECT MakeRect( int, int, int, int );
int Load_Bmp( HDC, const TCHAR * );

// game.cpp
bool InitGame( void );
void Draw( HDC );
void DrawWall( void );
void Goal( void );


#endif

// EOF
