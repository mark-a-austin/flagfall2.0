#include "communication.hpp"

void setup() {
    // put your setup code here, to run once:
    Serial.begin(115200);
}

uint8_t buffer[512];

void loop() {
    if (Serial.available()) {
        size_t read_amnt = Serial.readBytes(buffer, 512);
        Operation op(buffer, read_amnt);
        if (op.kind == OpKind::Sensor) {
            uint64_t rsw_data = __UINT32_MAX__; // Get sensor data
            write_sensor_data(rsw_data);
        } else if (op.kind == OpKind::Magnet) {
            make_move_seq(op);
            write_ack();
        } else if (op.kind == OpKind::Led) {
            delay(100);
            write_ack();
        }
    }
}

size_t write_sensor_data(const uint64_t value) {
    Serial.write((uint8_t *) &value, sizeof(value));
}


void make_move_seq(Operation &op) {
    const uint8_t *curr_ptr = op.data; 
    while (curr_ptr < op.data + op.data_len()) {
        float x = *(float *) buffer;
        curr_ptr += 4;
        float y = *(float *) buffer; 
        curr_ptr += 4;
        bool magnet = (*buffer) != 0;
        curr_ptr += 1; // ??

        if (magnet) {
            delay(10); // Magnet on
        } else {
            delay(10); // Magnet off
        }
        delay(3000); // Move to board position (x, y)
    }
    delay(10); // Magnet off after move
}   







