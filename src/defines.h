// defines.h

#ifndef DEFINITIONS_HEADER
#define DEFINITIONS_HEADER


// ==============================================
// マクロの定義
// ==============================================

#define LEFT(pt) ( (pt).x == -1 && (pt).y == 0 )
#define FRONT(pt) ( (pt).x == 0 && (pt).y == -1 )
#define RIGHT(pt) ( (pt).x == 1 && (pt).y == 0 )
#define BACK(pt) ( (pt).x == 0 && (pt).y == 1 )

#define ID_FORW 100
#define ID_LEFT 101
#define ID_RIGHT 102
#define ID_BACK 103



#endif


// EOF


