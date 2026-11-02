# =============================================================================
# ctdozlite iOS / iPadOS / iPhone Duo Build System (Minimum iOS 12.0+)
# Self-contained inside ./ios/ — zero modifications to desktop workspace files
# =============================================================================

APP_NAME       := ctdozlite
VERSION        := 0.0.5
MIN_IOS        := 12.0
BUNDLE_ID      := net.ctdo.ctdozlite.ios

IOS_SDK        := $(shell xcrun --sdk iphoneos --show-sdk-path)
SIM_SDK        := $(shell xcrun --sdk iphonesimulator --show-sdk-path)
CLANG_IOS      := $(shell xcrun --sdk iphoneos --find clang)
CLANG_SIM      := $(shell xcrun --sdk iphonesimulator --find clang)
IBTOOL         := $(shell xcrun --find ibtool)

BUILD_DIR      := build
DEVICE_APP     := $(BUILD_DIR)/device/$(APP_NAME).app
SIM_APP        := $(BUILD_DIR)/sim/$(APP_NAME).app
IPA_OUT        := $(BUILD_DIR)/$(APP_NAME)-$(VERSION)-ios$(MIN_IOS)-universal.ipa

OBJC_SOURCES   := App/main.m App/CTDOBridge.m App/CTDOMainViewController.m
OBJC_HEADERS   := App/CTDOBridge.h App/CTDOMainViewController.h rust-core/include/ctdozlite_ios.h

RUST_DEV_LIB   := rust-core/target/aarch64-apple-ios/release/libctdozlite_ios_core.a
RUST_SIM_LIB   := rust-core/target/aarch64-apple-ios-sim/release/libctdozlite_ios_core.a

COMMON_FLAGS   := -fobjc-arc -O2 -Wall -Wextra -Wno-unused-parameter -Irust-core/include
FRAMEWORKS     := -framework UIKit -framework Foundation -framework CoreGraphics \
                  -framework QuartzCore -framework CoreImage -framework Security \
                  -lresolv

.PHONY: all rust-core device sim ipa test clean

all: rust-core device sim ipa

rust-core:
	@echo "=== [1/4] Building Rust Core Static Library (IPHONEOS_DEPLOYMENT_TARGET=$(MIN_IOS)) ==="
	cd rust-core && IPHONEOS_DEPLOYMENT_TARGET=$(MIN_IOS) cargo build --release --target aarch64-apple-ios
	cd rust-core && IPHONEOS_DEPLOYMENT_TARGET=$(MIN_IOS) cargo build --release --target aarch64-apple-ios-sim

device: rust-core $(OBJC_SOURCES) $(OBJC_HEADERS) App/Info.plist App/LaunchScreen.storyboard
	@echo "=== [2/4] Compiling Native iOS Device Bundle (arm64, min iOS $(MIN_IOS)) ==="
	@mkdir -p "$(DEVICE_APP)"
	"$(CLANG_IOS)" -arch arm64 -isysroot "$(IOS_SDK)" \
		-mios-version-min=$(MIN_IOS) \
		$(COMMON_FLAGS) $(OBJC_SOURCES) $(RUST_DEV_LIB) $(FRAMEWORKS) \
		-o "$(DEVICE_APP)/$(APP_NAME)"
	@cp App/Info.plist "$(DEVICE_APP)/Info.plist"
	@printf "APPL????" > "$(DEVICE_APP)/PkgInfo"
	"$(IBTOOL)" --compile "$(DEVICE_APP)/LaunchScreen.storyboardc" \
		--minimum-deployment-target $(MIN_IOS) App/LaunchScreen.storyboard
	@python3 generate_icons.py "$(DEVICE_APP)"
	@codesign --force --sign - --timestamp=none "$(DEVICE_APP)"
	@echo "✅ Device bundle ready: $(DEVICE_APP)"

sim: rust-core $(OBJC_SOURCES) $(OBJC_HEADERS) App/Info.plist App/LaunchScreen.storyboard
	@echo "=== [3/4] Compiling Native iOS Simulator Bundle (arm64-sim, min iOS $(MIN_IOS)) ==="
	@mkdir -p "$(SIM_APP)"
	"$(CLANG_SIM)" -arch arm64 -isysroot "$(SIM_SDK)" \
		-miphonesimulator-version-min=$(MIN_IOS) \
		$(COMMON_FLAGS) $(OBJC_SOURCES) $(RUST_SIM_LIB) $(FRAMEWORKS) \
		-o "$(SIM_APP)/$(APP_NAME)"
	@cp App/Info.plist "$(SIM_APP)/Info.plist"
	@printf "APPL????" > "$(SIM_APP)/PkgInfo"
	"$(IBTOOL)" --compile "$(SIM_APP)/LaunchScreen.storyboardc" \
		--minimum-deployment-target $(MIN_IOS) App/LaunchScreen.storyboard
	@python3 generate_icons.py "$(SIM_APP)"
	@codesign --force --sign - --timestamp=none "$(SIM_APP)"
	@echo "✅ Simulator bundle ready: $(SIM_APP)"

ipa: device
	@echo "=== [4/4] Packaging Unsigned/Ad-Hoc .ipa Installer ($(IPA_OUT)) ==="
	@rm -rf "$(BUILD_DIR)/Payload" "$(IPA_OUT)"
	@mkdir -p "$(BUILD_DIR)/Payload"
	@cp -R "$(DEVICE_APP)" "$(BUILD_DIR)/Payload/$(APP_NAME).app"
	@cd "$(BUILD_DIR)" && zip -qry "$(notdir $(IPA_OUT))" Payload
	@rm -rf "$(BUILD_DIR)/Payload"
	@ls -lh "$(IPA_OUT)"

test: all
	@echo "=== [TEST 1/3] Running Rust Core Unit Tests ==="
	cd rust-core && cargo test
	@echo "=== [TEST 2/3] Verifying Mach-O Minimum OS Version == $(MIN_IOS) ==="
	@otool -l "$(DEVICE_APP)/$(APP_NAME)" | grep -A 4 "LC_BUILD_VERSION"
	@otool -l "$(DEVICE_APP)/$(APP_NAME)" | grep -A 4 "LC_BUILD_VERSION" | grep -q "minos 12.0"
	@echo "✅ Verified LC_BUILD_VERSION minos == 12.0 on arm64 device binary"
	@echo "=== [TEST 3/3] Running Simulator Headless Self-Test (--self-test) ==="
	@SIM_UDID=$$(xcrun simctl list devices available | grep -E "iPhone.*\(Booted\)" | head -n1 | sed -E 's/.*\(([0-9A-F-]+)\) \(Booted\).*/\1/'); \
	if [ -z "$$SIM_UDID" ]; then \
		SIM_UDID=$$(xcrun simctl list devices available | grep -E "iPhone " | head -n1 | sed -E 's/.*\(([0-9A-F-]+)\).*/\1/'); \
		xcrun simctl boot "$$SIM_UDID" 2>/dev/null || true; \
		xcrun simctl bootstatus "$$SIM_UDID" -b; \
	fi; \
	xcrun simctl spawn "$$SIM_UDID" "$(abspath $(SIM_APP)/$(APP_NAME))" --self-test

clean:
	rm -rf "$(BUILD_DIR)"
	cd rust-core && cargo clean
