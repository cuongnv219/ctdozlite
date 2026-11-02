#import <UIKit/UIKit.h>
#import "CTDOBridge.h"

NS_ASSUME_NONNULL_BEGIN

@interface CTDOMainViewController : UIViewController
@property (nonatomic, assign) CTDOLayoutMode preferredLayoutMode;
@property (nonatomic, assign) BOOL duoDualChatMode; // YES = Left Chat #1 + Hinge + Right Chat #2; NO = Left ThreadList + Hinge + Right Chat
- (void)refreshAllData;
- (void)triggerGitHubUpdateCheck:(BOOL)simulateOldVersion;
@end

NS_ASSUME_NONNULL_END
