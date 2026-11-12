/*
 * File:        output/cpp/snippets/s4.cpp
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>

#include <proton/connection.hpp>
#include <proton/container.hpp>
#include <proton/delivery.hpp>
#include <proton/message.hpp>
#include <proton/messaging_handler.hpp>
#include <proton/receiver.hpp>

#include <nlohmann/json.hpp>

/**
 * Scenario S4 — Distinguish sync vs delta messages and reassemble a sync batch.
 *
 * Demonstrates: reading the x-message-mode AMQP application
 * property (A3) to tell snapshot data from change events; using the
 * x-sync-id / x-sync-sequence /
 * x-sync-total triple (A5) to detect when a sync batch is
 * complete.
 *
 * Sync progress is tracked in a `std::map<std::string, std::set<int>>`
 * keyed by x-sync-id; each set entry corresponds to one
 * x-sync-sequence value seen on the wire.
 */
class S4 : public proton::messaging_handler {
    std::string url_;
    std::string channel_;
    int max_messages_;
    int received_ = 0;
    // sync-id -> set of sequence numbers received so far.
    std::map<std::string, std::set<int>> sync_progress_;

public:
    S4(std::string url, std::string channel, int max_messages)
        : url_(std::move(url)),
          channel_(std::move(channel)),
          max_messages_(max_messages) {}

    void on_container_start(proton::container& c) override {
        c.connect(url_);
    }

    void on_connection_open(proton::connection& conn) override {
        conn.open_receiver(channel_);
        // Readiness handshake: the harness waits for this
        // before starting the publisher.
        std::cerr << "[ready]" << std::endl;
    }

    void on_message(proton::delivery& d, proton::message& msg) override {
        std::string mode = string_property(msg, "x-message-mode", "delta");
        std::string body = body_as_string(msg);
        nlohmann::json payload;
        try {
            payload = nlohmann::json::parse(body);
        } catch (const nlohmann::json::parse_error& e) {
            std::cerr << "[S4] JSON parse error: " << e.what() << std::endl;
            advance_or_close(d);
            return;
        }

        if (mode == "delta") {
            std::cout << "[delta] " << payload.dump() << std::endl;
            advance_or_close(d);
            return;
        }

        // mode == "sync" — track batch completion.
        std::string sync_id = string_property(msg, "x-sync-id", "");
        int seq = int_property(msg, "x-sync-sequence", 0);
        int total = int_property(msg, "x-sync-total", 0);

        if (sync_id.empty() || total == 0) {
            std::cout << "[sync*] " << payload.dump()
                      << "  (incomplete sync metadata)" << std::endl;
            advance_or_close(d);
            return;
        }

        sync_progress_[sync_id].insert(seq);
        const int progress = static_cast<int>(sync_progress_[sync_id].size());
        std::cout << "[sync " << sync_id << " " << progress << "/" << total
                  << "] " << payload.dump() << std::endl;
        if (progress == total) {
            std::cout << "[sync " << sync_id << "] complete" << std::endl;
            sync_progress_.erase(sync_id);
        }
        advance_or_close(d);
    }

private:
    void advance_or_close(proton::delivery& d) {
        ++received_;
        if (received_ >= max_messages_) {
            d.connection().close();
        }
    }

    static std::string string_property(const proton::message& msg,
                                       const std::string& key,
                                       const std::string& fallback) {
        try {
            const proton::message::property_map& props = msg.properties();
            if (!props.exists(key)) return fallback;
            return proton::get<std::string>(props.get(key));
        } catch (const std::exception&) {
            return fallback;
        }
    }

    static int int_property(const proton::message& msg,
                            const std::string& key,
                            int fallback) {
        try {
            const proton::message::property_map& props = msg.properties();
            if (!props.exists(key)) return fallback;
            const proton::value& v = props.get(key);
            try { return static_cast<int>(proton::get<int32_t>(v)); }
            catch (...) {}
            try { return static_cast<int>(proton::get<int64_t>(v)); }
            catch (...) {}
            try { return std::stoi(proton::get<std::string>(v)); }
            catch (...) {}
            return fallback;
        } catch (const std::exception&) {
            return fallback;
        }
    }

    static std::string body_as_string(const proton::message& msg) {
        const proton::value& body = msg.body();
        if (body.empty()) return std::string();
        try { return proton::get<std::string>(body); }
        catch (const proton::conversion_error&) {
            try {
                proton::binary b = proton::get<proton::binary>(body);
                return std::string(b.begin(), b.end());
            } catch (const proton::conversion_error&) {
                return proton::to_string(body);
            }
        }
    }
};

int main(int argc, char* argv[]) {
    (void)argc;
    (void)argv;

    const char* env_url = std::getenv("TOPAS_AMQP_URL");
    std::string url = env_url ? env_url : "amqps://localhost:5671";

    const char* env_max = std::getenv("TOPAS_MAX_MESSAGES");
    int max_messages = env_max ? std::atoi(env_max) : 20;
    if (max_messages <= 0) max_messages = 20;

    S4 handler(url, "detector.measurement", max_messages);
    proton::container c(handler);
    try {
        c.run();
    } catch (const std::exception& e) {
        std::cerr << "[S4] AMQP error: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
