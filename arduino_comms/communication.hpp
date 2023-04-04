#include "opcode.h"
#include <Arduino.h>
#include <FastLED.h>

OpKind get_opkind(const uint8_t* serial_read_buffer); 
bool handshake(); 

/**
 * @brief 
 * Data structure for providing a view into a stored instruction. 
 */
typedef struct Operation {
public: 
    const OpKind         kind; 
    const uint8_t *const data; 

    Operation() = delete; 
    Operation(const uint8_t* instruction, const size_t instr_len) 
    : _data_len(instr_len - 1), 
      _data_base(instruction + 1), 
      kind(get_opkind(instruction)), 
      data(_data_base) {}
    ~Operation() = default; 

    size_t data_len() { return _data_len; }
    CRGB* try_into_CRGB(); 
private: 
    const uint8_t *const _data_base; 
    const size_t         _data_len; 
}; 

/**
 * @brief 
 * Checks the first byte of an instruction to determine its operation variant. 
 * 
 * @param serial_read_buffer Array of bytes representing the read instruction.
 * @return Corresponding `OpKind` enumeration type. 
 */
OpKind get_opkind(const uint8_t* serial_read_buffer) {
    switch(*serial_read_buffer) {
        case SENSOR: 
            return Sensor; 
        case MAGNET: 
            return Magnet; 
        case LED: 
            return Led; 
        case QUIT: 
            return Quit; 
        default: 
            return Noop; 
    }
}

void write_ack() {
    uint8_t buf[1] {ACK}; 
    Serial.write(buf, sizeof(buf)); 
}