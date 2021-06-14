#pragma once

#include "rust/cxx.h"
#include <memory>
#include "enums_forwards.h"

namespace amq
{
    class producer_consumer
    {
    public:
        //consturctor
        producer_consumer(amq_connection_type t);

        //destructor
        virtual ~producer_consumer();

        void run() const;
        void send_message(rust::String pMsg, int32_t pPriority) const;
        void close() const;

        rust::String get_last_error() const;
        amq_connection_type get_connection_type() const { return this->conn_type; }

        void set_broker_uri(rust::String p) const;
        void set_username(rust::String p) const;
        void set_password(rust::String p) const;
        void set_queue_or_topic_name(rust::String p) const;
        void set_pipeline_type(amq_pipeline_type p) const;
        void set_delivery_mode(amq_delivery_mode p) const;
        void set_session_transacted_mode(bool p) const;
        void set_on_message_recieved_callback(rust::Fn<void(rust::string)> pFN) const;

    private:
        //private constructors for avoid copy constructor
        producer_consumer(const producer_consumer &) = delete;
        producer_consumer &operator=(const producer_consumer &) = delete;

        class pimpl;
        class cimpl;
        std::shared_ptr<pimpl> p_impl;
        std::shared_ptr<cimpl> c_impl;
        amq_connection_type conn_type;
    };
    std::unique_ptr<producer_consumer> new_instance(amq_connection_type t);
} // amq