// game.cpp

#include <time.h>
#include <windows.h>
#include <tchar.h>

#include "resource.h"
#include "structs.h"
#include "funcs.h"
#include "vars.h"


namespace
{
  constexpr auto LEFT(POINT const& pt) { return pt.x == -1 && pt.y == 0; }
  constexpr auto FRONT(POINT const& pt) { return pt.x == 0 && pt.y == -1; }
  constexpr auto RIGHT(POINT const& pt) { return pt.x == 1 && pt.y == 0; }
  constexpr auto BACK(POINT const& pt) { return pt.x == 0 && pt.y == 1; }

  constexpr auto MINIX = 256 + 48 * 2;
  constexpr auto MINIY = 48;

  unsigned char* map;  // マップデータ
  int mapw, maph;      // マップの幅と高さ

  PLAYER pl;           // プレイヤー情報

  bool bMMap;          // ミニマップ表示フラグ

  int nmap;            // マップ呼び出し回数
  int nkey;            // キー押下回数

  HDC hBk;
  HBITMAP hBkBm;

  HDC hWall;
  HBITMAP hWallBm;

  HDC hMini;
  HBITMAP hMnBm;

  bool CreateMap();
}


// ゲームの初期化
bool InitGame( void )
{
  srand( ( unsigned int )time( NULL ) );

  RelsSurface( hBk, hBkBm );
  InitSurface( hBk, hBkBm, 256, 256 );

  RelsSurface( hWall, hWallBm );
  InitSurface( hWall, hWallBm, 1024, 256 );
  Load_Bmp( hWall, _T("wall.bmp") );

  CreateMap();

  RelsSurface( hMini, hMnBm );
  InitSurface( hMini, hMnBm, mapw, maph );
  PaintRect( hMini, MakeRect( 0, 0, mapw, maph ), RGB( 255, 255, 255 ) );
  for ( int i = 0; i < mapw; ++i )
  {
    for ( int j = 0; j < maph; ++j )
    {
      if ( map[i + j * mapw] == MI_NOTHING )
        //PaintRect( hMini, MakeRect( i * MINIZ, j * MINIZ, MINIZ, MINIZ ), 0 );
        SetPixel( hMini, i, j, RGB( 0, 0, 0 ) );
    }
  }
  /*PaintRect( hMini, MakeRect( 2 * MINIZ, 2 * MINIZ, MINIZ, MINIZ ), RGB( 0, 255, 255 ) );
  PaintRect( hMini, MakeRect( ( mapw - 3 ) * MINIZ, ( maph - 3 ) * MINIZ, MINIZ, MINIZ ), RGB( 255, 0, 0 ) );*/
  SetPixel( hMini, 2, 2, RGB( 0, 255, 255 ) );
  SetPixel( hMini, mapw - 3, maph - 3, RGB( 255, 0, 0 ) );

  pl.x = 2;
  pl.y = 2;
  pl.dp.x = 0;
  pl.dp.y = 1;
  bMMap = false;
  nkey = 0;
  nmap = 0;

  DrawWall();

  return true;
}

void ReleaseGame()
{
  RelsSurface( hMini, hMnBm );
  delete [] map;
  RelsSurface( hWall, hWallBm );
  RelsSurface( hBk, hBkBm );
}


auto turnLeft() -> void
{
  auto const tmp = pl.dp.x;
  pl.dp.x = pl.dp.y;
  pl.dp.y = -tmp;
  ++nkey;
  DrawWall();
}

auto moveForward() -> void
{
  if ( map[pl.x + pl.dp.x + ( pl.y + pl.dp.y ) * mapw] == MI_WALL )
    return;

  pl.x += pl.dp.x;
  pl.y += pl.dp.y;
  if ( pl.x < 0 ) pl.x = 0;
  if ( pl.x > ( mapw - 1 ) ) pl.x = mapw - 1;
  if ( pl.y < 0 ) pl.y = 0;
  if ( pl.y > ( maph - 1 ) ) pl.y = maph - 1;
  ++nkey;
  DrawWall();
}

auto turnRight() -> void
{
  auto const tmp = pl.dp.y;
  pl.dp.y = pl.dp.x;
  pl.dp.x = -tmp;
  ++nkey;
  DrawWall();
}

auto turnBack() -> void
{
  pl.dp.x = -pl.dp.x;
  pl.dp.y = -pl.dp.y;
  ++nkey;
  DrawWall();
}

auto toggleMap() -> void
{
  bMMap = !bMMap;
  --nkey;
  ++nmap;
}


// メイン描画
void Draw( HDC hDC )
{
  if (!map) return;

  Rectangle( hDC, 48 - 1, 48 - 1, 256 + 48 + 1, 256 + 48 + 1 );
  BitBlt( hDC, 48, 48, 256, 256, hBk, 0, 0, SRCCOPY );

  Rectangle( hDC, 48 * 2 + 256 - 1, 48 - 1,
    48 * 2 + 256 * 2 + 1, 256 + 48 + 1 );

  if ( bMMap )
  {
    //BitBlt( hDC, MINIX, MINIY, mapw * MINIZ, maph * MINIZ, hMini, 0, 0, SRCCOPY );
    StretchBlt( hDC, MINIX, MINIY, 256, 256, hMini, 0, 0, mapw, maph, SRCCOPY );
    POINT pt[3];
    pt[0].x = MINIX + pl.x * 67 / 6 + 5 * ( pl.dp.y ? 1 : 0 ) + ( 1 - pl.dp.x ) * 5 * ( pl.dp.x ? 1 : 0 );
    pt[0].y = MINIY + pl.y * 67 / 6 + ( 1 + pl.dp.y ) * 5 + 5 * -pl.dp.x;
    pt[1].x = MINIX + pl.x * 67 / 6 + ( pl.dp.x > 0 ? 10 : 0 );
    pt[1].y = MINIY + pl.y * 67 / 6 + ( 1 - pl.dp.y ) * 5;
    pt[2].x = MINIX + pl.x * 67 / 6 + 10 * ( pl.dp.x > 0 ? 0 : 1 );
    pt[2].y = MINIY + pl.y * 67 / 6 + ( 1 - pl.dp.y ) * 5 + 5 * pl.dp.x;
    Polygon( hDC, pt, 3 );
    //PaintRect( hDC, MakeRect( MINIX + pl.x * 67 / 6, MINIY + pl.y * 67 / 6, 67 / 6, 67 / 6 ), RGB( 200, 200, 0 ) );
  }

  PaintRect( hDC, MakeRect( 20, 48 + 256 + 12, 300, 20 ), RGB( 255, 255, 255 ) );
  TextOut( hDC, 20, 48 + 256 + 12, _T("移動：矢印キー マップ：Mキー 終了：ESCキー"), 24 );
  /*TCHAR szPos[32];
  PatBlt( hDC, 0, 0, 500, 20, WHITENESS );
  wsprintf( szPos, _T("(%d, %d)"), pl.x, pl.y );
  if ( LEFT( pl.dp ) ) lstrcat( szPos, _T("　左") );
  if ( FRONT( pl.dp ) ) lstrcat( szPos, _T("　前") );
  if ( RIGHT( pl.dp ) ) lstrcat( szPos, _T("　右") );
  if ( BACK( pl.dp ) ) lstrcat( szPos, _T("　後") );
  TextOut( hDC, 0, 0, szPos, lstrlen( szPos ) );*/

  // ゴール処理
  if ( pl.x == ( mapw - 3 ) && pl.y == ( maph - 3 ) )
    Goal();
}

// 壁描画
void DrawWall( void )
{
  unsigned char sight[17];

  BitBlt( hBk, 0, 0, 256, 256, hWall, 0, 0, SRCCOPY );
  FillMemory( sight, 10, MI_WALL );

  // 10 11 12 13 14 15 16
  //     0  1  2  3  4
  //        5  6  7
  //        8  I  9

  if ( LEFT( pl.dp ) )
  {
    if ( pl.x - 3 >= 0 )
    {
      sight[10] = ( pl.y + 4 > maph ? MI_WALL : map[pl.x - 3 + ( pl.y + 3 ) * mapw] );
      sight[11] = ( pl.y + 3 > maph ? MI_WALL : map[pl.x - 3 + ( pl.y + 2 ) * mapw] );
      sight[12] = map[pl.x - 3 + ( pl.y + 1 ) * mapw];
      sight[13] = map[pl.x - 3 + pl.y * mapw];
      sight[14] = map[pl.x - 3 + ( pl.y - 1 ) * mapw];
      sight[15] = ( pl.y < 2 ? MI_WALL : map[pl.x - 3 + ( pl.y - 2 ) * mapw] );
      sight[16] = ( pl.y < 3 ? MI_WALL : map[pl.x - 3 + ( pl.y - 3 ) * mapw] );

      sight[0] = ( pl.y + 3 > maph ? MI_WALL : map[pl.x - 2 + ( pl.y + 2 ) * mapw] );
      sight[1] = map[pl.x - 2 + ( pl.y + 1 ) * mapw];
      sight[2] = map[pl.x - 2 + pl.y * mapw];
      sight[3] = map[pl.x - 2 + ( pl.y - 1 ) * mapw];
      sight[4] = ( pl.y < 2 ? MI_WALL : map[pl.x - 2 + ( pl.y - 2 ) * mapw] );
    }
    else if ( pl.x - 2 >= 0 )
    {
      sight[0] = ( pl.y + 3 > maph ? MI_WALL : map[pl.x - 2 + ( pl.y + 2 ) * mapw] );
      sight[1] = map[pl.x - 2 + ( pl.y + 1 ) * mapw];
      sight[2] = map[pl.x - 2 + pl.y * mapw];
      sight[3] = map[pl.x - 2 + ( pl.y - 1 ) * mapw];
      sight[4] = ( pl.y < 2 ? MI_WALL : map[pl.x - 2 + ( pl.y - 2 ) * mapw] );
    }

    sight[5] = map[pl.x - 1 + ( pl.y + 1 ) * mapw];
    sight[6] = map[pl.x - 1 + pl.y * mapw];
    sight[7] = map[pl.x - 1 + ( pl.y - 1 ) * mapw];

    sight[8] = map[pl.x + ( pl.y + 1 ) * mapw];
    sight[9] = map[pl.x + ( pl.y - 1 ) * mapw];
  }

  if ( FRONT( pl.dp ) )
  {
    if ( pl.y - 2 >= 0 )
    {
      sight[10] = ( pl.x < 3 ? MI_WALL : map[pl.x - 3 + ( pl.y - 3 ) * mapw] );
      sight[11] = ( pl.x < 2 ? MI_WALL : map[pl.x - 2 + ( pl.y - 3 ) * mapw] );
      sight[12] = map[pl.x - 1 + ( pl.y - 3 ) * mapw];
      sight[13] = map[pl.x + ( pl.y - 3 ) * mapw];
      sight[14] = map[pl.x + 1 + ( pl.y - 3 ) * mapw];
      sight[15] = ( pl.x + 3 > mapw ? MI_WALL : map[pl.x + 2 + ( pl.y - 3 ) * mapw] );
      sight[16] = ( pl.x + 4 > mapw ? MI_WALL : map[pl.x + 3 + ( pl.y - 3 ) * mapw] );

      sight[0] = ( pl.x < 2 ? MI_WALL : map[pl.x - 2 + ( pl.y - 2 ) * mapw] );
      sight[1] = map[pl.x - 1 + ( pl.y - 2 ) * mapw];
      sight[2] = map[pl.x + ( pl.y - 2 ) * mapw];
      sight[3] = map[pl.x + 1 + ( pl.y - 2 ) * mapw];
      sight[4] = ( pl.x + 3 > mapw ? MI_WALL : map[pl.x + 2 + ( pl.y - 2 ) * mapw] );
    }
    else if ( pl.y - 2 >= 0 )
    {
      sight[0] = ( pl.x < 2 ? MI_WALL : map[pl.x - 2 + ( pl.y - 2 ) * mapw] );
      sight[1] = map[pl.x - 1 + ( pl.y - 2 ) * mapw];
      sight[2] = map[pl.x + ( pl.y - 2 ) * mapw];
      sight[3] = map[pl.x + 1 + ( pl.y - 2 ) * mapw];
      sight[4] = ( pl.x + 3 > mapw ? MI_WALL : map[pl.x + 2 + ( pl.y - 2 ) * mapw] );
    }

    sight[5] = map[pl.x - 1 + ( pl.y - 1 ) * mapw];
    sight[6] = map[pl.x + ( pl.y - 1 ) * mapw];
    sight[7] = map[pl.x + 1 + ( pl.y - 1 ) * mapw];

    sight[8] = map[pl.x - 1 + pl.y * mapw];
    sight[9] = map[pl.x + 1 + pl.y * mapw];
  }

  if ( RIGHT( pl.dp ) )
  {
    if ( pl.x + 3 < mapw )
    {
      sight[10] = ( pl.y < 3 ? MI_WALL : map[pl.x + 3 + ( pl.y - 3 ) * mapw] );
      sight[11] = ( pl.y < 2 ? MI_WALL : map[pl.x + 3 + ( pl.y - 2 ) * mapw] );
      sight[12] = map[pl.x + 3 + ( pl.y - 1 ) * mapw];
      sight[13] = map[pl.x + 3 + pl.y * mapw];
      sight[14] = map[pl.x + 3 + ( pl.y + 1 ) * mapw];
      sight[15] = ( pl.y + 3 > maph ? MI_WALL : map[pl.x + 3 + ( pl.y + 2 ) * mapw] );
      sight[16] = ( pl.y + 4 > maph ? MI_WALL : map[pl.x + 3 + ( pl.y + 3 ) * mapw] );

      sight[0] = ( pl.y < 2 ? MI_WALL : map[pl.x + 2 + ( pl.y - 2 ) * mapw] );
      sight[1] = map[pl.x + 2 + ( pl.y - 1 ) * mapw];
      sight[2] = map[pl.x + 2 + pl.y * mapw];
      sight[3] = map[pl.x + 2 + ( pl.y + 1 ) * mapw];
      sight[4] = ( pl.y + 3 > maph ? MI_WALL : map[pl.x + 2 + ( pl.y + 2 ) * mapw] );
    }
    else if ( pl.x + 3 < mapw )
    {
      sight[0] = ( pl.y < 2 ? MI_WALL : map[pl.x + 2 + ( pl.y - 2 ) * mapw] );
      sight[1] = map[pl.x + 2 + ( pl.y - 1 ) * mapw];
      sight[2] = map[pl.x + 2 + pl.y * mapw];
      sight[3] = map[pl.x + 2 + ( pl.y + 1 ) * mapw];
      sight[4] = ( pl.y + 3 > maph ? MI_WALL : map[pl.x + 2 + ( pl.y + 2 ) * mapw] );
    }

    sight[5] = map[pl.x + 1 + ( pl.y - 1 ) * mapw];
    sight[6] = map[pl.x + 1 + pl.y * mapw];
    sight[7] = map[pl.x + 1 + ( pl.y + 1 ) * mapw];

    sight[8] = map[pl.x + ( pl.y - 1 ) * mapw];
    sight[9] = map[pl.x + ( pl.y + 1 ) * mapw];
  }

  if ( BACK( pl.dp ) )
  {
    if ( pl.y + 3 < maph )
    {
      sight[10] = ( pl.x + 4 > mapw ? MI_WALL : map[pl.x + 3 + ( pl.y + 3 ) * mapw] );
      sight[11] = ( pl.x + 3 > mapw ? MI_WALL : map[pl.x + 2 + ( pl.y + 3 ) * mapw] );
      sight[12] = map[pl.x + 1 + ( pl.y + 3 ) * mapw];
      sight[13] = map[pl.x + ( pl.y + 3 ) * mapw];
      sight[14] = map[pl.x - 1 + ( pl.y + 3 ) * mapw];
      sight[15] = ( pl.x < 2 ? MI_WALL : map[pl.x - 2 + ( pl.y + 3 ) * mapw] );
      sight[16] = ( pl.x < 3 ? MI_WALL : map[pl.x - 3 + ( pl.y + 3 ) * mapw] );

      sight[0] = ( pl.x + 3 > mapw ? MI_WALL : map[pl.x + 2 + ( pl.y + 2 ) * mapw] );
      sight[1] = map[pl.x + 1 + ( pl.y + 2 ) * mapw];
      sight[2] = map[pl.x + ( pl.y + 2 ) * mapw];
      sight[3] = map[pl.x - 1 + ( pl.y + 2 ) * mapw];
      sight[4] = ( pl.x < 2 ? MI_WALL : map[pl.x - 2 + ( pl.y + 2 ) * mapw] );
    }
    else if ( pl.y + 3 < maph )
    {
      sight[0] = ( pl.x + 3 > mapw ? MI_WALL : map[pl.x + 2 + ( pl.y + 2 ) * mapw] );
      sight[1] = map[pl.x + 1 + ( pl.y + 2 ) * mapw];
      sight[2] = map[pl.x + ( pl.y + 2 ) * mapw];
      sight[3] = map[pl.x - 1 + ( pl.y + 2 ) * mapw];
      sight[4] = ( pl.x < 2 ? MI_WALL : map[pl.x - 2 + ( pl.y + 2 ) * mapw] );
    }

    sight[5] = map[pl.x + 1 + ( pl.y + 1 ) * mapw];
    sight[6] = map[pl.x + ( pl.y + 1 ) * mapw];
    sight[7] = map[pl.x - 1 + ( pl.y + 1 ) * mapw];

    sight[8] = map[pl.x + 1 + pl.y * mapw];
    sight[9] = map[pl.x - 1 + pl.y * mapw];
  }

  // 三つ前
  if ( sight[10] == MI_WALL )
    BitBlt( hBk, 0, 108, 40 - 12, 40, hWall, 876 + 12, 108, SRCCOPY );

  if ( sight[11] == MI_WALL )
    BitBlt( hBk, 28, 108, 40, 40, hWall, 876, 108, SRCCOPY );

  if ( sight[12] == MI_WALL )
    BitBlt( hBk, 68, 108, 40, 40, hWall, 876, 108, SRCCOPY );

  if ( sight[14] == MI_WALL )
    BitBlt( hBk, 148, 108, 40, 40, hWall, 876, 108, SRCCOPY );

  if ( sight[15] == MI_WALL )
    BitBlt( hBk, 188, 108, 40, 40, hWall, 876, 108, SRCCOPY );

  if ( sight[16] == MI_WALL )
    BitBlt( hBk, 228, 108, 256 - 228, 40, hWall, 876, 108, SRCCOPY );

  if ( sight[13] == MI_WALL )
    BitBlt( hBk, 108, 108, 40, 40, hWall, 876, 108, SRCCOPY );

  // 二つ前
  if ( sight[0] == MI_WALL )
    BitBlt( hBk, 0, 72, 21, 112, hWall, 768, 72, SRCCOPY );

  if ( sight[1] == MI_WALL )
    BitBlt( hBk, 21, 72, 89, 112, hWall, 788, 72, SRCCOPY );

  if ( sight[3] == MI_WALL )
    BitBlt( hBk, 147, 72, 89, 112, hWall, 915, 72, SRCCOPY );

  if ( sight[4] == MI_WALL )
    BitBlt( hBk, 235, 72, 21, 112, hWall, 1003, 72, SRCCOPY );

  if ( sight[2] == MI_WALL )
    BitBlt( hBk, 72, 72, 112, 112, hWall, 584, 72, SRCCOPY );

  // 一つ前
  if ( sight[5] == MI_WALL )
    BitBlt( hBk, 0, 20, 73, 216, hWall, 512, 20, SRCCOPY );

  if ( sight[7] == MI_WALL )
    BitBlt( hBk, 183, 20, 73, 216, hWall, 695, 20, SRCCOPY );

  if ( sight[6] == MI_WALL )
    BitBlt( hBk, 20, 20, 216, 216, hWall, 276, 20, SRCCOPY );

  // 目の前
  if ( sight[8] == MI_WALL )
    BitBlt( hBk, 0, 0, 20, 256, hWall, 256, 0, SRCCOPY );

  if ( sight[9] == MI_WALL )
    BitBlt( hBk, 236, 0, 20, 256, hWall, 492, 0, SRCCOPY );
}

// ゴール処理
void Goal( void )
{
  TCHAR szMes[0x100];
  int iRet;

  Mes( _T("ゴール！"), _T("Congratulations!") );

  wsprintf( szMes, _T("ゴールまでに%d回キーを押しました"), nkey );
  Mes( szMes, _T("結果発表ー") );

  wsprintf( szMes, _T("ちなみに%d回マップを出し消ししました"), nmap );
  Mes( szMes, _T("結果発表ー") );

  if ( nmap == 0 )
    Mes( _T("根性だね！"), _T("賞賛に値するよ") );
  else if ( nmap > 10 )
    Mes( _T("何がしたいの？"), _T("マップ連打バカ？") );

  Mes( _T("以上の結果からあなたの総合得点は……"), _T("計算中") );
  iRet = 5000 / nkey + ( nmap ? 10 / nmap : 50 ) + rand() % 30;

  wsprintf( szMes, _T("%d点です！"), iRet );
  Mes( szMes, _T("わーわー（ぱちぱち）") );

  wsprintf( szMes, _T("ちなみに無理矢理文字にすると '%c' です。覚えとけ！"), iRet );
  Mes( szMes, _T("本気にするなよ") );

  iRet = Mes( _T("もっかいやる？"), _T("どうする？　ア（以下、自主規制）"), MB_YESNO );
  if ( iRet == IDYES )
  {
    if ( !InitGame() )
    {
      Mes( _T("ごめん失敗") );
      DestroyWindow( ghWnd );
    }
  }
  else
    DestroyWindow( ghWnd );
}


namespace
{
  // マップ作成
  bool CreateMap()
  {
    int i, j, tmp;

    mapw = 23;
    maph = 23;
    map = new unsigned char [mapw * maph];
    FillMemory( map, mapw * maph, MI_WALL );

    for ( i = 2; i < ( maph - 2 ); ++i )
    {
      for ( j = 2; j < ( mapw - 2 ); ++j )
      {
        if ( i % 2 && j % 2 ) continue;
        map[j + i * mapw] = MI_NOTHING;
      }
    }

    for ( i = 3; i < ( maph - 3 ); i += 2 )
    {
      tmp = rand() % 2;
      if ( tmp == 0 )
      {
        tmp = ( rand() % 2 ? 1 : -1 );
        map[3 + tmp + i * mapw] = MI_WALL;
      }
      else
      {
        tmp = ( rand() % 2 ? 1 : -1 );
        map[3 + ( i + tmp ) * mapw] = MI_WALL;
      }
    }

    for ( i = 5; i < ( mapw - 3 ); i += 2 )
    {
      for ( j = 3; j < ( maph - 3 ); j += 2 )
      {
        tmp = rand() % 2;
        if ( tmp == 0 )
        {
          map[i + 1 + j * mapw] = MI_WALL;
        }
        else
        {
          tmp = ( rand() % 2 ? 1 : -1 );
          map[i + ( j + tmp ) * mapw] = MI_WALL;
        }
      }
    }

    return true;
  }
}
