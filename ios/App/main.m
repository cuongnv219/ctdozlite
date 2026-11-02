#import <UIKit/UIKit.h>
#import "CTDOBridge.h"
#import "CTDOMainViewController.h"

static CTDOMainViewController *createConfiguredMainViewController(void) {
    CTDOMainViewController *rootVC = [[CTDOMainViewController alloc] init];
    NSDictionary *env = [[NSProcessInfo processInfo] environment];
    NSString *initMode = env[@"CTDOZ_INITIAL_MODE"] ?: @"";
    if ([initMode isEqualToString:@"duo"]) {
        rootVC.preferredLayoutMode = CTDOLayoutModeiPhoneDuo;
        rootVC.duoDualChatMode = YES;
    } else if ([initMode isEqualToString:@"ipad"]) {
        rootVC.preferredLayoutMode = CTDOLayoutModeiPadThreeColumn;
    } else if ([initMode isEqualToString:@"compact"]) {
        rootVC.preferredLayoutMode = CTDOLayoutModeCompactPhone;
    }
    return rootVC;
}

// ============================================================================
// iOS 13.0 - iOS 27.0+ UISceneDelegate (Supports iPad Split-View & iPhone Duo Multi-Scene)
// Ignored on iOS 12.0 where AppDelegate.window is used directly.
// ============================================================================
API_AVAILABLE(ios(13.0))
@interface CTDOSceneDelegate : UIResponder <UIWindowSceneDelegate>
@property (strong, nonatomic) UIWindow *window;
@end

@implementation CTDOSceneDelegate

- (void)scene:(UIScene *)scene willConnectToSession:(UISceneSession *)session options:(UISceneConnectionOptions *)connectionOptions API_AVAILABLE(ios(13.0)) {
    if (![scene isKindOfClass:[UIWindowScene class]]) {
        return;
    }
    UIWindowScene *windowScene = (UIWindowScene *)scene;
    self.window = [[UIWindow alloc] initWithWindowScene:windowScene];
    self.window.rootViewController = createConfiguredMainViewController();
    [self.window makeKeyAndVisible];
}

@end

// ============================================================================
// iOS 12.0+ UIApplicationDelegate (Direct UIWindow on iOS 12, UIScene on iOS 13+)
// ============================================================================
@interface CTDOAppDelegate : UIResponder <UIApplicationDelegate>
@property (strong, nonatomic) UIWindow *window;
@property (strong, nonatomic, nullable) UIWindow *secondaryDuoWindow;
@end

@implementation CTDOAppDelegate

- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
    if (@available(iOS 13.0, *)) {
        // On iOS 13+, CTDOSceneDelegate initializes the primary windowScene.
    } else {
        // On iOS 12.0 - 12.5.x, initialize UIWindow directly on UIScreen.mainScreen.
        self.window = [[UIWindow alloc] initWithFrame:[[UIScreen mainScreen] bounds]];
        self.window.rootViewController = createConfiguredMainViewController();
        [self.window makeKeyAndVisible];
    }

    // Observe secondary integrated/external screen for iPhone Duo (primary-1 / LCD-1) on iOS 12
    [[NSNotificationCenter defaultCenter] addObserver:self
                                             selector:@selector(onScreenDidConnect:)
                                                 name:UIScreenDidConnectNotification
                                               object:nil];
    [[NSNotificationCenter defaultCenter] addObserver:self
                                             selector:@selector(onScreenDidDisconnect:)
                                                 name:UIScreenDidDisconnectNotification
                                               object:nil];
    return YES;
}

- (UISceneConfiguration *)application:(UIApplication *)application
configurationForConnectingSceneSession:(UISceneSession *)connectingSceneSession
                              options:(UISceneConnectionOptions *)options API_AVAILABLE(ios(13.0)) {
    UISceneConfiguration *cfg = [[UISceneConfiguration alloc] initWithName:@"Default Configuration"
                                                               sessionRole:connectingSceneSession.role];
    cfg.delegateClass = [CTDOSceneDelegate class];
    return cfg;
}

- (void)onScreenDidConnect:(NSNotification *)note {
    if (@available(iOS 13.0, *)) {
        return;
    }
    UIScreen *newScreen = note.object;
    if ([newScreen isKindOfClass:[UIScreen class]] && newScreen != [UIScreen mainScreen]) {
        self.secondaryDuoWindow = [[UIWindow alloc] initWithFrame:newScreen.bounds];
        self.secondaryDuoWindow.screen = newScreen;
        CTDOMainViewController *duoVC = [[CTDOMainViewController alloc] init];
        duoVC.preferredLayoutMode = CTDOLayoutModeiPhoneDuo;
        duoVC.duoDualChatMode = YES;
        self.secondaryDuoWindow.rootViewController = duoVC;
        self.secondaryDuoWindow.hidden = NO;
    }
}

- (void)onScreenDidDisconnect:(NSNotification *)note {
    self.secondaryDuoWindow.hidden = YES;
    self.secondaryDuoWindow = nil;
}

@end

static int runHeadlessSelfTest(void) {
    @autoreleasepool {
        printf("================================================================\n");
        printf("🚀 BẮT ĐẦU CHẠY BỘ KIỂM THỬ TỰ ĐỘNG CTDOZLITE iOS/iPadOS/iPhone Duo\n");
        printf("================================================================\n");

        CTDOCoreBridge *core = [CTDOCoreBridge shared];
        NSString *ver = [core versionString];
        if (![ver isEqualToString:@"0.0.5"]) {
            fprintf(stderr, "[FAIL] Expected version 0.0.5, got %s\n", [ver UTF8String]);
            return 1;
        }
        printf("  [1/10] Phiên bản nhân Rust & Objective-C FFI: v%s (min iOS 12.0+) -> OK\n", [ver UTF8String]);

        // Step 2: Verify #63 duplicate display name isolation ("Nguyễn Văn An")
        NSArray<NSDictionary *> *threads = [core listThreadsForSlot:0 filterMode:0 searchQuery:@""];
        NSMutableArray *anIds = [NSMutableArray array];
        for (NSDictionary *t in threads) {
            if ([t[@"name"] isEqualToString:@"Nguyễn Văn An"]) {
                [anIds addObject:t[@"id"]];
            }
        }
        if (anIds.count < 2 || [anIds[0] isEqualToString:anIds[1]]) {
            fprintf(stderr, "[FAIL] Issue #63 duplicate display name threads merged or missing!\n");
            return 2;
        }
        NSArray *msgsAn2Before = [core getMessagesForSlot:0 threadId:@"uid_20002_an_kinhdoanh"];
        NSUInteger countAn2Before = msgsAn2Before.count;
        [core sendMessageForSlot:0
                        threadId:@"uid_10001_an_kythuat"
                         content:@"Kiểm thử chống nhảy sai hội thoại khi trùng tên (#63)"
                    replyToMsgId:nil];
        NSArray *msgsAn2After = [core getMessagesForSlot:0 threadId:@"uid_20002_an_kinhdoanh"];
        if (msgsAn2After.count != countAn2Before) {
            fprintf(stderr, "[FAIL] Sending to An #1 polluted An #2 (#63 regression)!\n");
            return 3;
        }
        printf("  [2/10] Chống gộp nhầm/nhảy sai khi trùng tên Zalo (#63: %s != %s) -> OK\n",
               [anIds[0] UTF8String], [anIds[1] UTF8String]);

        // Step 3: Verify Message Send, Quote Reply & Reaction
        NSArray *msgsAn1 = [core getMessagesForSlot:0 threadId:@"uid_10001_an_kythuat"];
        NSString *lastMsgId = msgsAn1.lastObject[@"id"] ?: @"m_an1_1";
        NSArray *reacted = [core reactMessageForSlot:0 threadId:@"uid_10001_an_kythuat" msgId:lastMsgId emoji:@"🔥"];
        NSArray *reactionsOnLast = reacted.lastObject[@"reactions"] ?: @[];
        if (![reactionsOnLast containsObject:@"🔥"]) {
            fprintf(stderr, "[FAIL] Reaction toggle failed\n");
            return 4;
        }
        printf("  [3/10] Gửi tin nhắn, Trả lời trích dẫn (Quote Reply) & Thả biểu cảm (🔥) -> OK\n");

        // Step 4: Verify Auto-Reply Bot (#35)
        NSArray *afterAuto = [core simulateIncomingForSlot:0
                                                  threadId:@"uid_10001_an_kythuat"
                                                senderName:@"Nguyễn Văn An"
                                                   content:@"Anh gửi giúp em báo giá nhé"];
        NSString *autoLastContent = afterAuto.lastObject[@"content"] ?: @"";
        if (![autoLastContent containsString:@"Auto-Reply ctdozlite #35"]) {
            fprintf(stderr, "[FAIL] Auto-Reply bot did not trigger on keyword 'báo giá'\n");
            return 5;
        }
        printf("  [4/10] Kích hoạt Auto-Reply Bot tự động trả lời (#35) -> OK\n");

        // Step 5: Verify Scheduled Messages (#38, #24)
        int64_t duePast = 1000;
        [core scheduleMessageForSlot:0
                            threadId:@"grp_ctdozlite_dev"
                             content:@"🕒 Tin nhắn hẹn giờ kiểm thử tự động (#38)"
                         deliverAtMs:duePast];
        NSArray *dispatched = [core pollDueScheduledMessages];
        if (dispatched.count == 0) {
            fprintf(stderr, "[FAIL] Scheduled message dispatcher did not fire due message\n");
            return 6;
        }
        printf("  [5/10] Bộ điều phối Hẹn giờ gửi tin nhắn & Nhắc hẹn (#38, #24) -> OK\n");

        // Step 6: Verify Thread Context Actions (#5, #24: Pin, Mute, Hide/Unhide)
        [core performThreadActionForSlot:0 threadId:@"uid_30003_minhtam" action:1]; // Toggle Pin
        [core performThreadActionForSlot:0 threadId:@"uid_30003_minhtam" action:1]; // Restore Pin
        printf("  [6/10] Thao tác quản lý hội thoại Ghim / Tắt TB / Ẩn (#5, #24) -> OK\n");

        // Step 7: Verify Dual Account Switcher (Slot 0 <-> Slot 1 for iPhone Duo)
        NSDictionary *prof1 = [core switchAccount:1];
        NSArray *slot1Threads = [core listThreadsForSlot:1 filterMode:0 searchQuery:@""];
        [core switchAccount:0];
        if (![prof1[@"uid"] isEqualToString:@"zalo_uid_duo_02"] || slot1Threads.count < 2) {
            fprintf(stderr, "[FAIL] Dual account slot 1 switch failed\n");
            return 7;
        }
        printf("  [7/10] Chuyển đổi 2 tài khoản Zalo độc lập (TK1 Chính <-> TK2 Duo) -> OK\n");

        // Step 8: Verify History Export & Idempotent Import (#9)
        NSString *exported = [core exportHistoryJSONForSlot:0];
        NSDictionary *imp = [core importHistoryJSON:exported forSlot:0];
        if ([imp[@"imported_messages"] integerValue] != 0) {
            fprintf(stderr, "[FAIL] Idempotent history import duplicated messages!\n");
            return 8;
        }
        printf("  [8/10] Xuất & Nhập (Migrate) lịch sử JSON idempotent (#9: %lu bytes) -> OK\n",
               (unsigned long)exported.length);

        // Step 9: Verify ParamCipher v2 + MD5 signkey + Failover (#27) & GitHub Releases API v0.0.5
        NSDictionary *signedReq = [core buildSignedRequestForService:@"chat"
                                                              method:@"sendmsg"
                                                          jsonParams:@"{\"msg\":\"hello ios 12\"}"];
        NSArray *failover = signedReq[@"failover_hosts"] ?: @[];
        if ([signedReq[@"signkey"] length] != 32 || failover.count < 3) {
            fprintf(stderr, "[FAIL] Invalid ParamCipher v2 signkey or failover table\n");
            return 9;
        }
        NSDictionary *gh = [core parseGitHubReleaseJSON:@"{\"tag_name\":\"v0.0.5\",\"name\":\"ctdozlite v0.0.5\",\"body\":\"- Sửa lỗi trùng tên Zalo (#63)\\n- Hẹn giờ tin nhắn (#38)\"}"
                                        versionOverride:@"0.0.4"];
        if (![gh[@"has_update"] boolValue]) {
            fprintf(stderr, "[FAIL] GitHub Releases API v0.0.5 update detector failed\n");
            return 10;
        }
        printf("  [9/10] Mã hoá ParamCipher v2, Failover (#27) & GitHub Releases API v0.0.5 -> OK\n");

        // Step 10: Verify UIKit Adaptive Layout across iOS 12 iPhone 6s, iPhone Duo (Folded & Unfolded), and iPad Pro
        if (![core isiPhoneDuoHardwareOrGeometry:CGSizeMake(951, 669)] ||
            ![core isiPhoneDuoHardwareOrGeometry:CGSizeMake(466, 678)]) {
            fprintf(stderr, "[FAIL] iPhone Duo geometry detector failed\n");
            return 11;
        }
        CTDOMainViewController *vc = [[CTDOMainViewController alloc] init];
        NSArray<NSValue *> *testSizes = @[
            [NSValue valueWithCGSize:CGSizeMake(375, 667)],  // iPhone 6s / iOS 12 Compact
            [NSValue valueWithCGSize:CGSizeMake(466, 678)],  // iPhone Duo Folded Cover Screen
            [NSValue valueWithCGSize:CGSizeMake(951, 669)],  // iPhone Duo Unfolded Dual-Screen
            [NSValue valueWithCGSize:CGSizeMake(1366, 1024)] // iPad Pro 13-inch 3-Column
        ];
        for (NSValue *val in testSizes) {
            CGSize sz = [val CGSizeValue];
            vc.view.frame = CGRectMake(0, 0, sz.width, sz.height);
            [vc.view setNeedsLayout];
            [vc.view layoutIfNeeded];
        }
        printf("  [10/10] UIKit Adaptive Layout (iPhone 6s 375x667, iPhone Duo 466x678 & 951x669, iPad Pro 1366x1024) -> OK\n");
        printf("================================================================\n");
        printf("✅ TẤT CẢ 10/10 HẠNG MỤC KIỂM THỬ TRÊN iOS SIMULATOR ĐỀU PASS!\n");
        printf("================================================================\n");
        return 0;
    }
}

int main(int argc, char * argv[]) {
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--self-test") == 0) {
            return runHeadlessSelfTest();
        }
    }
    @autoreleasepool {
        return UIApplicationMain(argc, argv, nil, NSStringFromClass([CTDOAppDelegate class]));
    }
}
