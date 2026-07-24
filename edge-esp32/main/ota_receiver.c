#include "esp_ota_ops.h"
#include "esp_partition.h"
#include "esp_log.h"
#include "sodium.h" // For Ed25519 crypto_sign_verify_detached

static const char *TAG = "MESH_OTA";
static esp_ota_handle_t update_handle = 0;
static const esp_partition_t *update_partition = NULL;
static size_t bytes_written = 0;

// Hardcoded or WoT-synced trusted Orchestrator Public Key
extern const unsigned char ORCHESTRATOR_PUBLIC_KEY[crypto_sign_PUBLICKEYBYTES];

void begin_mesh_ota(const unsigned char *announced_hash, const unsigned char *signature, size_t total_size) {
    // 1. Verify Orchestrator Signature
    if (crypto_sign_verify_detached(signature, announced_hash, 32, ORCHESTRATOR_PUBLIC_KEY) != 0) {
        ESP_LOGE(TAG, "[!] OTA REJECTED: Invalid Ed25519 signature.");
        return;
    }

    // 2. Prepare Flash Partition
    update_partition = esp_ota_get_next_update_partition(NULL);
    if (update_partition == NULL) {
        ESP_LOGE(TAG, "[!] OTA Failed: No passive partition found.");
        return;
    }

    ESP_LOGI(TAG, "[*] Signature verified. Formatting partition: %s", update_partition->label);
    
    // OTA_SIZE_UNKNOWN allows writing until we manually end it
    esp_err_t err = esp_ota_begin(update_partition, OTA_SIZE_UNKNOWN, &update_handle);
    if (err != ESP_OK) {
        ESP_LOGE(TAG, "[!] esp_ota_begin failed (%s)", esp_err_to_name(err));
    }
    bytes_written = 0;
}

void process_ota_chunk(const uint8_t *data, size_t len) {
    if (update_handle == 0) return; // Not initialized

    esp_err_t err = esp_ota_write(update_handle, data, len);
    if (err != ESP_OK) {
        ESP_LOGE(TAG, "[!] OTA Write failed: %s", esp_err_to_name(err));
        return;
    }
    
    bytes_written += len;
    ESP_LOGI(TAG, "[+] Wrote chunk. Total: %d bytes", bytes_written);
}

void finalize_mesh_ota() {
    if (update_handle == 0) return;

    if (esp_ota_end(update_handle) != ESP_OK) {
        ESP_LOGE(TAG, "[!] OTA End failed.");
        return;
    }

    // Hash validation should occur here before setting the boot partition
    
    esp_err_t err = esp_ota_set_boot_partition(update_partition);
    if (err != ESP_OK) {
        ESP_LOGE(TAG, "[!] Failed to set boot partition: %s", esp_err_to_name(err));
        return;
    }

    ESP_LOGI(TAG, "[+] OTA Complete. Restarting into new firmware...");
    esp_restart();
}
