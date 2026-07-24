#include <string.h>
#include "esp_wifi.h"
#include "esp_now.h"
#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "nvs_flash.h"
#include "remote_viewer.pb.h" // Generated via nanopb
#include "pb_encode.h"

static const char *TAG = "REMOTE_VIEWER_EDGE";

// ESP-NOW receive callback for mesh relay
static void esp_now_recv_cb(const esp_now_recv_info_t *info, const uint8_t *data, int len) {
    ESP_LOGI(TAG, "Received mesh packet of size: %d", len);
}

void app_main(void) {
    // Initialize NVS
    esp_err_t ret = nvs_flash_init();
    if (ret == ESP_ERR_NVS_NO_FREE_PAGES || ret == ESP_ERR_NVS_NEW_VERSION_FOUND) {
        ESP_ERROR_CHECK(nvs_flash_erase());
        ret = nvs_flash_init();
    }
    ESP_ERROR_CHECK(ret);

    // Initialize Wi-Fi in station mode for ESP-NOW
    ESP_ERROR_CHECK(esp_netif_init());
    ESP_ERROR_CHECK(esp_event_loop_create_default());
    wifi_init_config_t cfg = WIFI_INIT_CONFIG_DEFAULT();
    ESP_ERROR_CHECK(esp_wifi_init(&cfg));
    ESP_ERROR_CHECK(esp_wifi_set_mode(WIFI_MODE_STA));
    ESP_ERROR_CHECK(esp_wifi_start());

    // Initialize ESP-NOW
    ESP_ERROR_CHECK(esp_now_init());
    ESP_ERROR_CHECK(esp_now_register_recv_cb(esp_now_recv_cb));

    ESP_LOGI(TAG, "Edge node active. Listening for local spectrum telemetry...");
    
    while (1) {
        vTaskDelay(pdMS_TO_Ticks(5000));
        // Periodic sensor polling and local broadcast loop can be inserted here
    }
}
