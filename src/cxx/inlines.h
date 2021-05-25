#pragma once

#include <activemq/library/ActiveMQCPP.h>

namespace amq
{
    void init()
    {
        activemq::library::ActiveMQCPP::initializeLibrary();
    }
    void fini()
    {
        activemq::library::ActiveMQCPP::shutdownLibrary();
    }
} // amq