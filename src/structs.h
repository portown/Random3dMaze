// structs.h

#ifndef STRUCTIONS_HEADER
#define STRUCTIONS_HEADER


// ==============================================
// 構造体etcの定義
// ==============================================

// 主人公情報構造体
struct PLAYER {
	int x;		// x座標
	int y;		// y座標
	POINT dp;	// 向き
};

// マップ情報列挙型
enum MAPINFO {
	MI_NOTHING = 0,
	MI_WALL,
};


#endif


// EOF


