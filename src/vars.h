// vars.h

#ifndef VARIABLES_HEADER
#define VARIABLES_HEADER

#ifdef MAIN_DECLARE
#define EX
#else
#define EX extern
#endif


// ==============================================
// 外部変数の宣言
// ==============================================

EX HWND ghWnd;          // グローバルウィンドウハンドル

EX int mapw, maph;      // マップの幅と高さ
EX unsigned char *map;  // マップデータ
EX bool bMMap;          // ミニマップ表示フラグ

EX PLAYER pl;           // プレイヤー情報

EX int nmap;            // マップ呼び出し回数
EX int nkey;            // キー押下回数


#endif


// EOF


