#include "producer_consumer.h"
#include <activemq/core/ActiveMQConnectionFactory.h>
#include <decaf/lang/Runnable.h>
#include "enums.h"

namespace amq
{
    class producer_consumer::cimpl : public cms::ExceptionListener,
                                     public cms::MessageListener,
                                     public decaf::lang::Runnable
    {
        friend producer_consumer;

    public:
        virtual void run()
        {
            using namespace cms;
            try
            {
                // Create a ConnectionFactory
                if (broker_uri.empty())
                {
                    throw CMSException("broker uri was not provided");
                }

                auto _connection_factory =
                    ConnectionFactory::createCMSConnectionFactory(this->broker_uri);

                //Create a Connection
                this->connection = _connection_factory->createConnection(
                    this->username,
                    this->password);
                this->connection->start();
                this->connection->setExceptionListener(this);

                // Create a Session
                if (this->session_transacted)
                {
                    this->session = this->connection->createSession(Session::SESSION_TRANSACTED);
                }
                else
                {
                    this->session = this->connection->createSession(Session::AUTO_ACKNOWLEDGE);
                }

                // Create the destination (Topic or Queue)
                if (this->pipeline_type == amq_pipeline_type::AMQ_QUEUE)
                {
                    if (queue_or_topic_name.empty())
                    {
                        throw CMSException("queue name was not provided");
                    }
                    this->destination = this->session->createQueue(this->queue_or_topic_name);
                }
                else
                {
                    if (queue_or_topic_name.empty())
                    {
                        throw CMSException("topic name was not provided");
                    }
                    this->destination = this->session->createTopic(this->queue_or_topic_name);
                }

                // Create a MessageProducer from the Session to the Topic or Queue
                this->consumer = this->session->createConsumer(this->destination);
                if (this->consumer)
                {
                    this->consumer->setMessageListener(this);
                }
                else
                {
                    this->last_error = "could not create activemq consumer";
                }
            }
            catch (const CMSException &ex)
            {
                onException(ex);
            }
        }

        virtual void onMessage(const cms::Message *pMessage)
        {
            using namespace cms;
            try
            {
                auto _text_message = dynamic_cast<const TextMessage *>(pMessage);
                if (_text_message)
                {
                    if (on_message_callback)
                    {
                        rust::string _msg = _text_message->getText();
                        (*on_message_callback)(_msg);
                    }
                }
                else
                {
                    this->last_error = "NULL message recieved";
                }
            }
            catch (const CMSException &ex)
            {
                onException(ex);
            }

            //commit all messages
            if (this->session_transacted)
            {
                this->session->commit();
            }
        }

        void close()
        {
            try
            {
                if (this->connection)
                {
                    this->connection->close();
                    delete this->connection;
                    this->connection = nullptr;
                }

                if (this->destination)
                {
                    delete this->destination;
                    this->destination = nullptr;
                }

                if (this->consumer)
                {
                    delete this->consumer;
                    this->consumer = nullptr;
                }

                if (this->session)
                {
                    delete this->session;
                    this->session = nullptr;
                }

                if (this->connection)
                {
                    delete this->connection;
                    this->connection = nullptr;
                }
            }
            catch (const cms::CMSException &e)
            {
                onException(e);
            }
        }

        std::string last_error;
        cms::Connection *connection = nullptr;
        cms::Session *session = nullptr;
        cms::Destination *destination = nullptr;
        cms::MessageConsumer *consumer = nullptr;
        std::string broker_uri;
        std::string username;
        std::string password;
        std::string queue_or_topic_name;
        amq_pipeline_type pipeline_type = amq_pipeline_type::AMQ_QUEUE;
        amq_delivery_mode delivery_mode = amq_delivery_mode::AMQ_PERSISTENT;
        bool session_transacted = false;
        rust::Fn<void(rust::string)> *on_message_callback = nullptr;

    private:
        //registered as an ExceptionListener with the connection.
        virtual void onException(const cms::CMSException &e)
        {
            this->last_error =
                "activemq::consumer exception occurred: " +
                e.getMessage() +
                " trace info: " +
                e.getStackTraceString();
        }
    };

    class producer_consumer::pimpl : public decaf::lang::Runnable
    {
        friend producer_consumer;

    public:
        virtual void run()
        {
            using namespace cms;
            try
            {
                // Create a ConnectionFactory
                if (broker_uri.empty())
                {
                    throw CMSException("broker uri was not provided");
                }

                auto _connection_factory =
                    ConnectionFactory::createCMSConnectionFactory(this->broker_uri);

                //Create a Connection
                this->connection = _connection_factory->createConnection(
                    this->username,
                    this->password);
                this->connection->start();

                // Create a Session
                if (this->session_transacted)
                {
                    this->session = this->connection->createSession(Session::SESSION_TRANSACTED);
                }
                else
                {
                    this->session = this->connection->createSession(Session::AUTO_ACKNOWLEDGE);
                }

                // Create the destination (Topic or Queue)
                if (this->pipeline_type == amq_pipeline_type::AMQ_QUEUE)
                {
                    if (queue_or_topic_name.empty())
                    {
                        throw CMSException("queue name was not provided");
                    }
                    this->destination = this->session->createQueue(this->queue_or_topic_name);
                }
                else
                {
                    if (queue_or_topic_name.empty())
                    {
                        throw CMSException("topic name was not provided");
                    }
                    this->destination = this->session->createTopic(this->queue_or_topic_name);
                }

                // Create a MessageProducer from the Session to the Topic or Queue
                this->producer = this->session->createProducer(this->destination);
                if (this->producer)
                {
                    this->producer->setDeliveryMode(static_cast<int>(this->delivery_mode));
                }
                else
                {
                    this->last_error = "could not create activemq producer";
                }
            }
            catch (const CMSException &ex)
            {
                onException(ex);
            }
        }

        void send_message(rust::str pMessage, int pPriority)
        {
            //make a copy from this message
            if (!pMessage.empty())
            {
                auto _message = this->session->createTextMessage(pMessage.data());
                _message->setIntProperty("Integer", pPriority);
                this->producer->send(_message);
            }
        }

        void close()
        {
            try
            {
                if (this->connection)
                {
                    this->connection->close();
                    delete this->connection;
                    this->connection = nullptr;
                }

                if (this->destination)
                {
                    delete this->destination;
                    this->destination = nullptr;
                }

                if (this->producer)
                {
                    delete this->producer;
                    this->producer = nullptr;
                }

                if (this->session)
                {
                    delete this->session;
                    this->session = nullptr;
                }

                if (this->connection)
                {
                    delete this->connection;
                    this->connection = nullptr;
                }
            }
            catch (const cms::CMSException &e)
            {
                onException(e);
            }
        }

        std::string last_error;
        cms::Connection *connection;
        cms::Session *session;
        cms::Destination *destination;
        cms::MessageProducer *producer;
        std::string broker_uri;
        std::string username;
        std::string password;
        std::string queue_or_topic_name;
        amq_pipeline_type pipeline_type;
        amq_delivery_mode delivery_mode;
        bool session_transacted;

    private:
        //registered as an ExceptionListener with the connection.
        virtual void onException(const cms::CMSException &e)
        {
            this->last_error =
                "activemq::producer exception occurred: " +
                e.getMessage() +
                " trace info: " +
                e.getStackTraceString();
        }
    };

    producer_consumer::producer_consumer(amq_connection_type t) : conn_type(t)
    {
        if (this->conn_type == amq_connection_type::AMQ_CONSUMER)
        {
            c_impl = std::make_shared<producer_consumer::cimpl>();
            p_impl = nullptr;
        }
        else
        {
            p_impl = std::make_shared<producer_consumer::pimpl>();
            c_impl = nullptr;
        }
    }

    producer_consumer::~producer_consumer()
    {
        (this->conn_type == amq_connection_type::AMQ_CONSUMER) ? c_impl->close() : p_impl->close();
#ifdef DEBUG
        std::cout << "producer_consumer destructor called" << std::endl;
#endif
    }

    void producer_consumer::run() const
    {
        (this->conn_type == amq_connection_type::AMQ_CONSUMER) ? c_impl->run() : p_impl->run();
    }

    void producer_consumer::send_message(rust::str pMessage, int pPriority) const
    {
        if (this->conn_type == amq_connection_type::AMQ_PRODUCER)
        {
            p_impl->send_message(pMessage, pPriority);
        }
    }

    void producer_consumer::close() const
    {
        (this->conn_type == amq_connection_type::AMQ_CONSUMER) ? c_impl->close() : p_impl->close();
    }

    rust::str producer_consumer::get_last_error() const
    {
        return rust::str(
            (this->conn_type == amq_connection_type::AMQ_CONSUMER) ? c_impl->last_error.data() : p_impl->last_error.data());
    }

    void producer_consumer::set_broker_uri(rust::str p) const
    {
        if (this->conn_type == amq_connection_type::AMQ_CONSUMER)
        {
            c_impl->broker_uri = p.data();
        }
        else
        {
            p_impl->broker_uri = p.data();
        }
    }

    void producer_consumer::set_user_name(rust::str p) const
    {
        if (this->conn_type == amq_connection_type::AMQ_CONSUMER)
        {
            c_impl->username = p.data();
        }
        else
        {
            p_impl->username = p.data();
        }
    }

    void producer_consumer::set_password(rust::str p) const
    {
        if (this->conn_type == amq_connection_type::AMQ_CONSUMER)
        {
            c_impl->password = p.data();
        }
        else
        {
            p_impl->password = p.data();
        }
    }

    void producer_consumer::set_queue_or_topic_name(rust::str p) const
    {
        if (this->conn_type == amq_connection_type::AMQ_CONSUMER)
        {
            c_impl->queue_or_topic_name = p.data();
        }
        else
        {
            p_impl->queue_or_topic_name = p.data();
        }
    }

    void producer_consumer::set_pipeline_type(amq_pipeline_type p) const
    {
        if (this->conn_type == amq_connection_type::AMQ_CONSUMER)
        {
            c_impl->pipeline_type = p;
        }
        else
        {
            p_impl->pipeline_type = p;
        }
    }

    void producer_consumer::set_delivery_mode(amq_delivery_mode p) const
    {
        if (this->conn_type == amq_connection_type::AMQ_CONSUMER)
        {
            c_impl->delivery_mode = p;
        }
        else
        {
            p_impl->delivery_mode = p;
        }
    }

    void producer_consumer::set_session_transacted_mode(bool p) const
    {
        if (this->conn_type == amq_connection_type::AMQ_CONSUMER)
        {
            c_impl->session_transacted = p;
        }
        else
        {
            p_impl->session_transacted = p;
        }
    }

    void producer_consumer::set_on_message_recieved_callback(rust::Fn<void(rust::string)> pFN) const
    {
        if (this->conn_type == amq_connection_type::AMQ_CONSUMER)
        {
            c_impl->on_message_callback = &pFN;
        }
    }

    std::unique_ptr<producer_consumer> new_instance(amq_connection_type t)
    {
        return std::make_unique<producer_consumer>(t);
    }
} //amq