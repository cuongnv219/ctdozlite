#import "CTDOBridge.h"
#import <CoreImage/CoreImage.h>
#include <sys/utsname.h>

@implementation CTDOTheme

+ (instancetype)shared {
    static CTDOTheme *instance = nil;
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
        instance = [[CTDOTheme alloc] init];
        NSInteger saved = [[NSUserDefaults standardUserDefaults] integerForKey:@"ctdoz_theme_mode"];
        instance.mode = (CTDOThemeMode)saved;
    });
    return instance;
}

- (void)setMode:(CTDOThemeMode)mode {
    _mode = mode;
    [[NSUserDefaults standardUserDefaults] setInteger:mode forKey:@"ctdoz_theme_mode"];
}

- (void)cycleTheme {
    self.mode = (CTDOThemeMode)((self.mode + 1) % 3);
}

- (NSString *)themeName {
    switch (self.mode) {
        case CTDOThemeModeZaloLight:
            return @"Zalo Light (Sáng)";
        case CTDOThemeModeOLEDMidnight:
            return @"OLED Midnight (Tiết kiệm pin)";
        case CTDOThemeModeZaloDark:
        default:
            return @"Zalo Dark (Mặc định)";
    }
}

- (UIColor *)bgPrimary {
    switch (self.mode) {
        case CTDOThemeModeZaloLight:
            return [UIColor colorWithRed:0.95 green:0.96 blue:0.98 alpha:1.0];
        case CTDOThemeModeOLEDMidnight:
            return [UIColor colorWithRed:0.01 green:0.01 blue:0.02 alpha:1.0];
        default:
            return [UIColor colorWithRed:0.055 green:0.078 blue:0.125 alpha:1.0];
    }
}

- (UIColor *)bgSecondary {
    switch (self.mode) {
        case CTDOThemeModeZaloLight:
            return [UIColor whiteColor];
        case CTDOThemeModeOLEDMidnight:
            return [UIColor colorWithRed:0.04 green:0.05 blue:0.08 alpha:1.0];
        default:
            return [UIColor colorWithRed:0.082 green:0.114 blue:0.180 alpha:1.0];
    }
}

- (UIColor *)bgElevated {
    switch (self.mode) {
        case CTDOThemeModeZaloLight:
            return [UIColor colorWithRed:0.91 green:0.93 blue:0.96 alpha:1.0];
        case CTDOThemeModeOLEDMidnight:
            return [UIColor colorWithRed:0.08 green:0.10 blue:0.15 alpha:1.0];
        default:
            return [UIColor colorWithRed:0.118 green:0.161 blue:0.239 alpha:1.0];
    }
}

- (UIColor *)bgHeader {
    switch (self.mode) {
        case CTDOThemeModeZaloLight:
            return [UIColor colorWithRed:0.00 green:0.41 blue:1.00 alpha:1.0];
        case CTDOThemeModeOLEDMidnight:
            return [UIColor colorWithRed:0.03 green:0.05 blue:0.09 alpha:1.0];
        default:
            return [UIColor colorWithRed:0.043 green:0.063 blue:0.106 alpha:1.0];
    }
}

- (UIColor *)textPrimary {
    if (self.mode == CTDOThemeModeZaloLight) {
        return [UIColor colorWithRed:0.07 green:0.09 blue:0.15 alpha:1.0];
    }
    return [UIColor colorWithRed:0.94 green:0.96 blue:0.99 alpha:1.0];
}

- (UIColor *)textSecondary {
    if (self.mode == CTDOThemeModeZaloLight) {
        return [UIColor colorWithRed:0.38 green:0.44 blue:0.53 alpha:1.0];
    }
    return [UIColor colorWithRed:0.58 green:0.64 blue:0.74 alpha:1.0];
}

- (UIColor *)accentPrimary {
    return [UIColor colorWithRed:0.00 green:0.45 blue:1.00 alpha:1.0];
}

- (UIColor *)accentOnline {
    return [UIColor colorWithRed:0.06 green:0.73 blue:0.51 alpha:1.0];
}

- (UIColor *)bubbleSelf {
    if (self.mode == CTDOThemeModeZaloLight) {
        return [UIColor colorWithRed:0.86 green:0.93 blue:1.00 alpha:1.0];
    }
    return [UIColor colorWithRed:0.05 green:0.32 blue:0.68 alpha:1.0];
}

- (UIColor *)bubblePartner {
    if (self.mode == CTDOThemeModeZaloLight) {
        return [UIColor whiteColor];
    }
    return [UIColor colorWithRed:0.118 green:0.161 blue:0.239 alpha:1.0];
}

- (UIColor *)borderSubtle {
    if (self.mode == CTDOThemeModeZaloLight) {
        return [UIColor colorWithRed:0.84 green:0.87 blue:0.92 alpha:1.0];
    }
    return [UIColor colorWithRed:0.16 green:0.22 blue:0.32 alpha:1.0];
}

- (UIColor *)dangerColor {
    return [UIColor colorWithRed:0.94 green:0.27 blue:0.27 alpha:1.0];
}

- (UIColor *)warningColor {
    return [UIColor colorWithRed:0.96 green:0.62 blue:0.04 alpha:1.0];
}

- (NSString *)exportThemeJSON {
    NSDictionary *dict = @{
        @"schema": @"ctdozlite.theme.v1",
        @"name": [self themeName],
        @"mode": @(self.mode),
        @"min_ios": @"12.0",
        @"accent_primary": @"#0073FF",
        @"accent_online": @"#10B981"
    };
    NSData *data = [NSJSONSerialization dataWithJSONObject:dict options:NSJSONWritingPrettyPrinted error:nil];
    return [[NSString alloc] initWithData:data encoding:NSUTF8StringEncoding] ?: @"{}";
}

+ (UIColor *)colorFromHex:(NSString *)hexString {
    NSString *clean = [[hexString stringByTrimmingCharactersInSet:[NSCharacterSet whitespaceAndNewlineCharacterSet]] uppercaseString];
    if ([clean hasPrefix:@"#"]) {
        clean = [clean substringFromIndex:1];
    }
    if (clean.length != 6) {
        return [UIColor colorWithRed:0.0 green:0.45 blue:1.0 alpha:1.0];
    }
    unsigned int rgb = 0;
    [[NSScanner scannerWithString:clean] scanHexInt:&rgb];
    return [UIColor colorWithRed:((rgb >> 16) & 0xFF) / 255.0
                           green:((rgb >> 8) & 0xFF) / 255.0
                            blue:(rgb & 0xFF) / 255.0
                           alpha:1.0];
}

@end

@implementation CTDOIconRenderer

+ (UIImage *)avatarWithInitials:(NSString *)initials
                       colorHex:(NSString *)colorHex
                           size:(CGFloat)size
                       isOnline:(BOOL)isOnline {
    static NSCache *avatarCache = nil;
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
        avatarCache = [[NSCache alloc] init];
        avatarCache.countLimit = 128;
    });
    NSString *cacheKey = [NSString stringWithFormat:@"%@_%@_%.0f_%d", initials, colorHex, size, isOnline];
    UIImage *cached = [avatarCache objectForKey:cacheKey];
    if (cached) {
        return cached;
    }

    UIGraphicsBeginImageContextWithOptions(CGSizeMake(size, size), NO, 0.0);
    CGContextRef ctx = UIGraphicsGetCurrentContext();

    UIColor *bg = [CTDOTheme colorFromHex:colorHex];
    [bg setFill];
    UIBezierPath *circle = [UIBezierPath bezierPathWithOvalInRect:CGRectMake(0, 0, size, size)];
    [circle fill];

    NSMutableParagraphStyle *pStyle = [[NSMutableParagraphStyle alloc] init];
    pStyle.alignment = NSTextAlignmentCenter;
    NSDictionary *attrs = @{
        NSFontAttributeName: [UIFont boldSystemFontOfSize:size * 0.38],
        NSForegroundColorAttributeName: [UIColor whiteColor],
        NSParagraphStyleAttributeName: pStyle
    };
    CGSize txtSize = [initials sizeWithAttributes:attrs];
    CGRect txtRect = CGRectMake(0, (size - txtSize.height) * 0.5, size, txtSize.height);
    [initials drawInRect:txtRect withAttributes:attrs];

    if (isOnline) {
        CGFloat dotSize = MAX(10.0, size * 0.26);
        CGRect dotRect = CGRectMake(size - dotSize - 1.0, size - dotSize - 1.0, dotSize, dotSize);
        CGContextSaveGState(ctx);
        [[[CTDOTheme shared] bgSecondary] setStroke];
        [[[CTDOTheme shared] accentOnline] setFill];
        UIBezierPath *dotPath = [UIBezierPath bezierPathWithOvalInRect:dotRect];
        dotPath.lineWidth = 2.0;
        [dotPath fill];
        [dotPath stroke];
        CGContextRestoreGState(ctx);
    }

    UIImage *img = UIGraphicsGetImageFromCurrentImageContext();
    UIGraphicsEndImageContext();
    if (img) {
        [avatarCache setObject:img forKey:cacheKey];
    }
    return img;
}

+ (UIImage *)qrCodeImageForString:(NSString *)content size:(CGFloat)size {
    NSData *data = [content dataUsingEncoding:NSUTF8StringEncoding];
    CIFilter *filter = [CIFilter filterWithName:@"CIQRCodeGenerator"];
    if (!filter) {
        return [self appLogoWithSize:size];
    }
    [filter setValue:data forKey:@"inputMessage"];
    [filter setValue:@"M" forKey:@"inputCorrectionLevel"];
    CIImage *ciImage = filter.outputImage;
    if (!ciImage) {
        return [self appLogoWithSize:size];
    }
    CGRect extent = CGRectIntegral(ciImage.extent);
    CGFloat scale = MIN(size / CGRectGetWidth(extent), size / CGRectGetHeight(extent));
    size_t width = (size_t)(CGRectGetWidth(extent) * scale);
    size_t height = (size_t)(CGRectGetHeight(extent) * scale);

    CIContext *ciCtx = [CIContext contextWithOptions:nil];
    CGImageRef cgImg = [ciCtx createCGImage:ciImage fromRect:extent];
    if (!cgImg) {
        return [self appLogoWithSize:size];
    }

    CGColorSpaceRef cs = CGColorSpaceCreateDeviceGray();
    CGContextRef bitmapRef = CGBitmapContextCreate(NULL, width, height, 8, 0, cs, (CGBitmapInfo)kCGImageAlphaNone);
    CGColorSpaceRelease(cs);
    CGContextSetInterpolationQuality(bitmapRef, kCGInterpolationNone);
    CGContextScaleCTM(bitmapRef, scale, scale);
    CGContextDrawImage(bitmapRef, extent, cgImg);
    CGImageRef scaledImg = CGBitmapContextCreateImage(bitmapRef);
    CGContextRelease(bitmapRef);
    CGImageRelease(cgImg);

    UIImage *result = [UIImage imageWithCGImage:scaledImg];
    CGImageRelease(scaledImg);
    return result;
}

+ (UIImage *)appLogoWithSize:(CGFloat)size {
    UIGraphicsBeginImageContextWithOptions(CGSizeMake(size, size), NO, 0.0);
    UIBezierPath *rounded = [UIBezierPath bezierPathWithRoundedRect:CGRectMake(0, 0, size, size)
                                                       cornerRadius:size * 0.24];
    [[UIColor colorWithRed:0.0 green:0.42 blue:1.0 alpha:1.0] setFill];
    [rounded fill];

    NSMutableParagraphStyle *pStyle = [[NSMutableParagraphStyle alloc] init];
    pStyle.alignment = NSTextAlignmentCenter;
    NSDictionary *attrs = @{
        NSFontAttributeName: [UIFont boldSystemFontOfSize:size * 0.44],
        NSForegroundColorAttributeName: [UIColor whiteColor],
        NSParagraphStyleAttributeName: pStyle
    };
    NSString *txt = @"Z";
    CGSize s = [txt sizeWithAttributes:attrs];
    [txt drawInRect:CGRectMake(0, (size - s.height) * 0.5, size, s.height) withAttributes:attrs];
    UIImage *img = UIGraphicsGetImageFromCurrentImageContext();
    UIGraphicsEndImageContext();
    return img;
}

@end

@implementation CTDOCoreBridge

+ (instancetype)shared {
    static CTDOCoreBridge *instance = nil;
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
        instance = [[CTDOCoreBridge alloc] init];
    });
    return instance;
}

- (instancetype)init {
    self = [super init];
    if (self) {
        NSArray *paths = NSSearchPathForDirectoriesInDomains(NSApplicationSupportDirectory, NSUserDomainMask, YES);
        NSString *supportDir = paths.firstObject ?: NSTemporaryDirectory();
        NSString *appDataDir = [supportDir stringByAppendingPathComponent:@"ctdozlite_ios"];
        [[NSFileManager defaultManager] createDirectoryAtPath:appDataDir
                                  withIntermediateDirectories:YES
                                                   attributes:@{NSFilePosixPermissions: @(0700)}
                                                        error:nil];
        ctdoz_ios_init([appDataDir UTF8String]);
    }
    return self;
}

- (NSString *)consumeRustString:(char *)ptr {
    if (!ptr) {
        return @"";
    }
    NSString *str = [NSString stringWithUTF8String:ptr] ?: @"";
    ctdoz_ios_free_string(ptr);
    return str;
}

- (id)parseJSONFromRustPtr:(char *)ptr fallback:(id)fallback {
    NSString *jsonStr = [self consumeRustString:ptr];
    if (jsonStr.length == 0) {
        return fallback;
    }
    NSData *data = [jsonStr dataUsingEncoding:NSUTF8StringEncoding];
    if (!data) {
        return fallback;
    }
    id obj = [NSJSONSerialization JSONObjectWithData:data options:0 error:nil];
    return obj ?: fallback;
}

- (NSString *)versionString {
    return [self consumeRustString:ctdoz_ios_version()];
}

- (BOOL)isiPhoneDuoHardwareOrGeometry:(CGSize)windowSize {
    // 1) Check simulator or hardware model identifier for iPhone Duo ("iPhone19,4" / "V68")
    NSDictionary *env = [[NSProcessInfo processInfo] environment];
    NSString *simModel = env[@"SIMULATOR_MODEL_IDENTIFIER"] ?: @"";
    NSString *simDevName = env[@"SIMULATOR_DEVICE_NAME"] ?: @"";
    if ([simModel isEqualToString:@"iPhone19,4"] ||
        [simModel isEqualToString:@"V68"] ||
        [simDevName localizedCaseInsensitiveContainsString:@"Duo"]) {
        return YES;
    }
    struct utsname sysinfo;
    if (uname(&sysinfo) == 0) {
        NSString *machine = [NSString stringWithUTF8String:sysinfo.machine];
        if ([machine isEqualToString:@"iPhone19,4"] || [machine isEqualToString:@"V68"]) {
            return YES;
        }
    }
    // 2) Check if multiple integrated screens are attached (iPhone Duo LCD + LCD-1)
    if ([UIScreen screens].count > 1) {
        return YES;
    }
    // 3) Check iPhone Duo unfolded geometry (951x669 pt or 669x951 pt) or folded (466x678 pt)
    CGFloat w = round(windowSize.width);
    CGFloat h = round(windowSize.height);
    if ((fabs(w - 951.0) <= 4.0 && fabs(h - 669.0) <= 4.0) ||
        (fabs(w - 669.0) <= 4.0 && fabs(h - 951.0) <= 4.0) ||
        (fabs(w - 466.0) <= 4.0 && fabs(h - 678.0) <= 4.0)) {
        return YES;
    }
    return NO;
}

- (NSDictionary *)switchAccount:(NSInteger)slot {
    return [self parseJSONFromRustPtr:ctdoz_ios_switch_account((int32_t)slot) fallback:@{}];
}

- (NSArray<NSDictionary *> *)listThreadsForSlot:(NSInteger)slot
                                     filterMode:(NSInteger)filterMode
                                    searchQuery:(nullable NSString *)query {
    const char *q = query ? [query UTF8String] : "";
    return [self parseJSONFromRustPtr:ctdoz_ios_list_threads_json((int32_t)slot, (int32_t)filterMode, q)
                             fallback:@[]];
}

- (NSArray<NSDictionary *> *)getMessagesForSlot:(NSInteger)slot
                                       threadId:(NSString *)threadId {
    return [self parseJSONFromRustPtr:ctdoz_ios_get_messages_json((int32_t)slot, [threadId UTF8String])
                             fallback:@[]];
}

- (NSArray<NSDictionary *> *)sendMessageForSlot:(NSInteger)slot
                                       threadId:(NSString *)threadId
                                        content:(NSString *)content
                                   replyToMsgId:(nullable NSString *)replyToMsgId {
    const char *rep = replyToMsgId ? [replyToMsgId UTF8String] : "";
    return [self parseJSONFromRustPtr:ctdoz_ios_send_message_json((int32_t)slot,
                                                                  [threadId UTF8String],
                                                                  [content UTF8String],
                                                                  rep)
                             fallback:@[]];
}

- (NSArray<NSDictionary *> *)simulateIncomingForSlot:(NSInteger)slot
                                            threadId:(NSString *)threadId
                                          senderName:(nullable NSString *)senderName
                                             content:(NSString *)content {
    const char *sname = senderName ? [senderName UTF8String] : "";
    return [self parseJSONFromRustPtr:ctdoz_ios_receive_message_json((int32_t)slot,
                                                                     [threadId UTF8String],
                                                                     sname,
                                                                     [content UTF8String])
                             fallback:@[]];
}

- (NSArray<NSDictionary *> *)reactMessageForSlot:(NSInteger)slot
                                        threadId:(NSString *)threadId
                                           msgId:(NSString *)msgId
                                           emoji:(NSString *)emoji {
    return [self parseJSONFromRustPtr:ctdoz_ios_react_message_json((int32_t)slot,
                                                                   [threadId UTF8String],
                                                                   [msgId UTF8String],
                                                                   [emoji UTF8String])
                             fallback:@[]];
}

- (NSArray<NSDictionary *> *)performThreadActionForSlot:(NSInteger)slot
                                               threadId:(NSString *)threadId
                                                 action:(NSInteger)action {
    return [self parseJSONFromRustPtr:ctdoz_ios_thread_action_json((int32_t)slot,
                                                                   [threadId UTF8String],
                                                                   (int32_t)action)
                             fallback:@[]];
}

- (NSArray<NSDictionary *> *)scheduleMessageForSlot:(NSInteger)slot
                                           threadId:(NSString *)threadId
                                            content:(NSString *)content
                                        deliverAtMs:(int64_t)deliverAtMs {
    return [self parseJSONFromRustPtr:ctdoz_ios_schedule_message_json((int32_t)slot,
                                                                      [threadId UTF8String],
                                                                      [content UTF8String],
                                                                      deliverAtMs)
                             fallback:@[]];
}

- (NSArray<NSDictionary *> *)listScheduledForSlot:(NSInteger)slot {
    return [self parseJSONFromRustPtr:ctdoz_ios_list_scheduled_json((int32_t)slot) fallback:@[]];
}

- (NSArray<NSDictionary *> *)cancelScheduledForSlot:(NSInteger)slot
                                            schedId:(int64_t)schedId {
    return [self parseJSONFromRustPtr:ctdoz_ios_cancel_scheduled_json((int32_t)slot, schedId)
                             fallback:@[]];
}

- (NSArray<NSDictionary *> *)pollDueScheduledMessages {
    int64_t nowMs = (int64_t)([[NSDate date] timeIntervalSince1970] * 1000.0);
    return [self parseJSONFromRustPtr:ctdoz_ios_poll_due_scheduled_json(nowMs) fallback:@[]];
}

- (NSArray<NSDictionary *> *)listAutoReplyRules {
    return [self parseJSONFromRustPtr:ctdoz_ios_list_autoreply_json() fallback:@[]];
}

- (NSArray<NSDictionary *> *)mutateAutoReplyOp:(NSInteger)op
                                   patternOrId:(NSString *)patternOrId
                                      response:(NSString *)response
                                     matchMode:(NSString *)matchMode {
    return [self parseJSONFromRustPtr:ctdoz_ios_mutate_autoreply_json((int32_t)op,
                                                                      [patternOrId UTF8String],
                                                                      [response UTF8String],
                                                                      [matchMode UTF8String])
                             fallback:@[]];
}

- (NSDictionary *)buildSignedRequestForService:(NSString *)service
                                        method:(NSString *)method
                                    jsonParams:(NSString *)jsonParams {
    return [self parseJSONFromRustPtr:ctdoz_ios_build_signed_request_json([service UTF8String],
                                                                          [method UTF8String],
                                                                          [jsonParams UTF8String],
                                                                          "ctdozlite_ios12_zpw_enk")
                             fallback:@{}];
}

- (NSDictionary *)parseGitHubReleaseJSON:(NSString *)rawJson
                         versionOverride:(nullable NSString *)versionOverride {
    const char *ver = versionOverride ? [versionOverride UTF8String] : "";
    return [self parseJSONFromRustPtr:ctdoz_ios_parse_github_release_json([rawJson UTF8String], ver)
                             fallback:@{}];
}

- (void)checkGitHubLatestReleaseWithSimulateOldVersion:(BOOL)simulateOld
                                            completion:(void (^)(NSDictionary *releaseInfo))completion {
    NSURL *url = [NSURL URLWithString:@"https://api.github.com/repos/ctdoteam1110/ctdozlite/releases/latest"];
    NSMutableURLRequest *req = [NSMutableURLRequest requestWithURL:url
                                                       cachePolicy:NSURLRequestReloadIgnoringLocalCacheData
                                                   timeoutInterval:8.0];
    [req setValue:@"application/vnd.github+json" forHTTPHeaderField:@"Accept"];
    [req setValue:@"ctdozlite-ios/0.0.5 (iOS 12.0+; iPhone; iPad; iPhone Duo)" forHTTPHeaderField:@"User-Agent"];

    NSString *verOverride = simulateOld ? @"0.0.4" : [self versionString];
    NSURLSessionDataTask *task = [[NSURLSession sharedSession]
        dataTaskWithRequest:req
          completionHandler:^(NSData * _Nullable data, NSURLResponse * _Nullable response, NSError * _Nullable error) {
              NSString *rawJson = nil;
              if (data && !error) {
                  rawJson = [[NSString alloc] initWithData:data encoding:NSUTF8StringEncoding];
              }
              if (!rawJson || rawJson.length < 10) {
                  // Offline / fallback snapshot of v0.0.5 release notes so update UI works offline too
                  rawJson = @"{\"tag_name\":\"v0.0.5\",\"name\":\"ctdozlite v0.0.5 — Tự động thông báo cập nhật qua GitHub API, Sửa lỗi trùng tên Zalo (#63), Hẹn giờ tin nhắn (#38) & REST/MCP API (#35)\",\"html_url\":\"https://github.com/ctdoteam1110/ctdozlite/releases/tag/v0.0.5\",\"published_at\":\"2026-11-02\",\"body\":\"### 🐛 Đã sửa lỗi\\n- **Sửa triệt để lỗi gộp nhầm/nhảy sai hội thoại khi tên Zalo trùng nhau (#63):** Tách biệt hoàn toàn theo `thread_id` thực tế.\\n- **Sửa lỗi xác thực gói cài đặt (#58, #20) & Failover đa máy chủ (#27):** Tự động chuyển đổi máy chủ dự phòng.\\n### ✨ Tính năng mới & Cập nhật\\n- **Tự động thông báo & cập nhật trực tiếp qua GitHub Releases API.**\\n- **Hẹn giờ gửi tin nhắn & Nhắc hẹn (#38, #24).**\\n- **Hỗ trợ iOS 12.0+, iPhone, iPad Split-View & iPhone Duo (Màn hình gập đôi / 2 tài khoản song song).**\"}";
              }
              NSDictionary *parsed = [self parseGitHubReleaseJSON:rawJson versionOverride:verOverride];
              dispatch_async(dispatch_get_main_queue(), ^{
                  if (completion) {
                      completion(parsed);
                  }
              });
          }];
    [task resume];
}

- (NSString *)exportHistoryJSONForSlot:(NSInteger)slot {
    return [self consumeRustString:ctdoz_ios_export_history_json((int32_t)slot)];
}

- (NSDictionary *)importHistoryJSON:(NSString *)jsonString forSlot:(NSInteger)slot {
    return [self parseJSONFromRustPtr:ctdoz_ios_import_history_json((int32_t)slot, [jsonString UTF8String])
                             fallback:@{}];
}

@end
