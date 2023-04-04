/*
 * Opcodes, written in pure C.
 */

// 0x00 passed to prevent sending NULL bytes
#define SENSOR    0x01
#define MAGNET    0x02
#define LED       0x03

#define HANDSHAKE 0x10
#define ACK       0x20
#define QUIT      0xFF



/**
 * @brief
 * Enumerates the variants of operations to be worked by the arduino main program. 
 */
typedef enum {
    Sensor, 
    Magnet, 
    Led, 
    Noop, 
    Quit
} OpKind; 