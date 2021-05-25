#pragma once

#include <stdint.h>

namespace amq
{
    enum class amq_connection_type : std::uint8_t
    {
        AMQ_CONSUMER = 0,
        AMQ_PRODUCER = 1,
    };
    enum class amq_pipeline_type : std::uint8_t
    {
        AMQ_QUEUE = 0,
        AMQ_TOPIC = 1,
    };
    enum class amq_delivery_mode : std::uint8_t
    {
        AMQ_PERSISTENT = 0,
        AMQ_NON_PERSISTENT = 1,
    };
} // amq