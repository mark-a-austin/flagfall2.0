#include <Arduino.h>
#include <FastLED.h>
#include "opcode.h"

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
      data(_data_base) {
    }
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

/**
 * @brief 
 * Parser for parsing an `Operation` of `Ops::Led` kind into a heap-allocated 
 * CRGB instance. 
 * 
 * @return `CRGB*` if instruction is of correct kind and is well-formed. 
 * @return `NULL` otherwise, or when out-of-memory. 
 */
CRGB* Operation::try_into_CRGB() {
    if (kind != OpKind::Led || data_len() != 3) {
        return NULL; 
    }
    return new CRGB(data[0], data[1], data[2]);  
}

/**
 * Perform handshake with Raspi host *once*. 
 * 
 * @return true if handshake is successful. 
 * @return false otherwise. 
 */
bool handshake() {
    /* Listen for PC initiation */
    if (Serial.available()) {
        size_t available_amnt = Serial.available(); 

        uint8_t buffer[available_amnt] {0}; 
        uint8_t arduino_id; 

        Serial.readBytes(buffer, available_amnt); 
        if (available_amnt == 2 && buffer[0] == HANDSHAKE) {
            // buffer[1..] should be Arduino ID, which is assumed to be non-zero
            arduino_id = buffer[1]; 
            Serial.write(buffer, available_amnt); 
        }
        /* Wait for PC readback */
        while (true) {
            if (Serial.available()) {
                memset(buffer, 0, available_amnt); 
                Serial.readBytes(buffer, available_amnt); 
                if (buffer[0] == HANDSHAKE && buffer[1] == arduino_id) {
                    Serial.write(ACK); 
                    return true; 
                }
            }
        }
    }
    return false; 
}