#import "CTDOMainViewController.h"

// ============================================================================
// Custom Reusable Thread TableView Cell (60/120fps, zero Nib overhead, iOS 12+)
// ============================================================================
@interface CTDOThreadCell : UITableViewCell
@property (nonatomic, strong) UIImageView *avatarView;
@property (nonatomic, strong) UILabel *nameLabel;
@property (nonatomic, strong) UILabel *tagBadgeLabel;
@property (nonatomic, strong) UILabel *subtitleLabel;
@property (nonatomic, strong) UILabel *timeLabel;
@property (nonatomic, strong) UILabel *unreadBadge;
@property (nonatomic, strong) UIView *separatorLine;
- (void)configureWithThread:(NSDictionary *)thread isSelected:(BOOL)isSelected;
@end

@implementation CTDOThreadCell

- (instancetype)initWithStyle:(UITableViewCellStyle)style reuseIdentifier:(NSString *)reuseIdentifier {
    self = [super initWithStyle:style reuseIdentifier:reuseIdentifier];
    if (self) {
        self.selectionStyle = UITableViewCellSelectionStyleNone;
        _avatarView = [[UIImageView alloc] initWithFrame:CGRectMake(12, 12, 48, 48)];
        _avatarView.contentMode = UIViewContentModeScaleAspectFit;
        [self.contentView addSubview:_avatarView];

        _nameLabel = [[UILabel alloc] init];
        _nameLabel.font = [UIFont boldSystemFontOfSize:15.0];
        [self.contentView addSubview:_nameLabel];

        _tagBadgeLabel = [[UILabel alloc] init];
        _tagBadgeLabel.font = [UIFont systemFontOfSize:11.0 weight:UIFontWeightMedium];
        [self.contentView addSubview:_tagBadgeLabel];

        _subtitleLabel = [[UILabel alloc] init];
        _subtitleLabel.font = [UIFont systemFontOfSize:13.0];
        _subtitleLabel.numberOfLines = 1;
        [self.contentView addSubview:_subtitleLabel];

        _timeLabel = [[UILabel alloc] init];
        _timeLabel.font = [UIFont systemFontOfSize:11.5];
        _timeLabel.textAlignment = NSTextAlignmentRight;
        [self.contentView addSubview:_timeLabel];

        _unreadBadge = [[UILabel alloc] init];
        _unreadBadge.font = [UIFont boldSystemFontOfSize:11.0];
        _unreadBadge.textColor = [UIColor whiteColor];
        _unreadBadge.backgroundColor = [UIColor colorWithRed:0.94 green:0.27 blue:0.27 alpha:1.0];
        _unreadBadge.textAlignment = NSTextAlignmentCenter;
        _unreadBadge.layer.cornerRadius = 9.0;
        _unreadBadge.layer.masksToBounds = YES;
        [self.contentView addSubview:_unreadBadge];

        _separatorLine = [[UIView alloc] init];
        [self.contentView addSubview:_separatorLine];
    }
    return self;
}

- (void)layoutSubviews {
    [super layoutSubviews];
    CGFloat w = self.contentView.bounds.size.width;
    CGFloat h = self.contentView.bounds.size.height;
    _avatarView.frame = CGRectMake(12, (h - 46) * 0.5, 46, 46);
    _timeLabel.frame = CGRectMake(w - 76, 10, 66, 16);
    _nameLabel.frame = CGRectMake(68, 9, MAX(80, w - 150), 19);
    _tagBadgeLabel.frame = CGRectMake(68, 29, MAX(80, w - 115), 15);
    _subtitleLabel.frame = CGRectMake(68, 45, MAX(80, w - 110), 18);
    _unreadBadge.frame = CGRectMake(w - 34, 42, 22, 18);
    _separatorLine.frame = CGRectMake(68, h - 0.5, w - 68, 0.5);
}

- (void)configureWithThread:(NSDictionary *)thread isSelected:(BOOL)isSelected {
    CTDOTheme *theme = [CTDOTheme shared];
    self.contentView.backgroundColor = isSelected ? theme.bgElevated : theme.bgSecondary;
    _separatorLine.backgroundColor = theme.borderSubtle;

    NSString *name = thread[@"name"] ?: @"Hội thoại";
    BOOL pinned = [thread[@"is_pinned"] boolValue];
    BOOL muted = [thread[@"is_muted"] boolValue];
    NSMutableString *titleStr = [NSMutableString stringWithString:name];
    if (pinned) [titleStr appendString:@" 📌"];
    if (muted) [titleStr appendString:@" 🔕"];
    _nameLabel.text = titleStr;
    _nameLabel.textColor = theme.textPrimary;

    _tagBadgeLabel.text = thread[@"subtitle_tag"] ?: @"";
    _tagBadgeLabel.textColor = theme.accentPrimary;

    _subtitleLabel.text = thread[@"last_message"] ?: @"";
    _subtitleLabel.textColor = theme.textSecondary;

    _timeLabel.text = thread[@"last_time_label"] ?: @"";
    _timeLabel.textColor = theme.textSecondary;

    NSInteger unread = [thread[@"unread_count"] integerValue];
    if (unread > 0) {
        _unreadBadge.hidden = NO;
        _unreadBadge.text = unread > 99 ? @"99+" : [NSString stringWithFormat:@"%ld", (long)unread];
    } else {
        _unreadBadge.hidden = YES;
    }

    NSString *initials = thread[@"avatar_initials"] ?: @"Z";
    NSString *hex = thread[@"avatar_color_hex"] ?: @"#0068FF";
    BOOL online = [thread[@"is_online"] boolValue];
    _avatarView.image = [CTDOIconRenderer avatarWithInitials:initials colorHex:hex size:46.0 isOnline:online];
}

@end

// ============================================================================
// Custom Reusable Chat Bubble Cell (Supports Quote Reply, Reactions, Metadata)
// ============================================================================
@interface CTDOMessageCell : UITableViewCell
@property (nonatomic, strong) UIView *bubbleView;
@property (nonatomic, strong) UILabel *senderLabel;
@property (nonatomic, strong) UILabel *quoteLabel;
@property (nonatomic, strong) UILabel *bodyLabel;
@property (nonatomic, strong) UILabel *metaLabel;
- (void)configureWithMessage:(NSDictionary *)msg maxWidth:(CGFloat)maxWidth;
+ (CGFloat)heightForMessage:(NSDictionary *)msg maxWidth:(CGFloat)maxWidth;
@end

@implementation CTDOMessageCell

- (instancetype)initWithStyle:(UITableViewCellStyle)style reuseIdentifier:(NSString *)reuseIdentifier {
    self = [super initWithStyle:style reuseIdentifier:reuseIdentifier];
    if (self) {
        self.selectionStyle = UITableViewCellSelectionStyleNone;
        self.backgroundColor = [UIColor clearColor];
        self.contentView.backgroundColor = [UIColor clearColor];

        _bubbleView = [[UIView alloc] init];
        _bubbleView.layer.cornerRadius = 14.0;
        _bubbleView.layer.borderWidth = 0.5;
        [self.contentView addSubview:_bubbleView];

        _senderLabel = [[UILabel alloc] init];
        _senderLabel.font = [UIFont boldSystemFontOfSize:11.5];
        [_bubbleView addSubview:_senderLabel];

        _quoteLabel = [[UILabel alloc] init];
        _quoteLabel.font = [UIFont italicSystemFontOfSize:11.5];
        _quoteLabel.numberOfLines = 2;
        _quoteLabel.layer.cornerRadius = 6.0;
        _quoteLabel.layer.masksToBounds = YES;
        [_bubbleView addSubview:_quoteLabel];

        _bodyLabel = [[UILabel alloc] init];
        _bodyLabel.font = [UIFont systemFontOfSize:14.5];
        _bodyLabel.numberOfLines = 0;
        [_bubbleView addSubview:_bodyLabel];

        _metaLabel = [[UILabel alloc] init];
        _metaLabel.font = [UIFont systemFontOfSize:11.0];
        [_bubbleView addSubview:_metaLabel];
    }
    return self;
}

+ (CGFloat)heightForMessage:(NSDictionary *)msg maxWidth:(CGFloat)maxWidth {
    CGFloat bubbleMaxW = MIN(maxWidth * 0.78, 460.0);
    CGFloat textMaxW = MAX(120.0, bubbleMaxW - 24.0);
    NSString *content = msg[@"content"] ?: @"";
    CGRect bodyRect = [content boundingRectWithSize:CGSizeMake(textMaxW, 2000.0)
                                            options:NSStringDrawingUsesLineFragmentOrigin
                                         attributes:@{NSFontAttributeName: [UIFont systemFontOfSize:14.5]}
                                            context:nil];
    CGFloat h = ceil(bodyRect.size.height) + 46.0;
    NSString *quote = msg[@"reply_preview"];
    if ([quote isKindOfClass:[NSString class]] && quote.length > 0) {
        h += 28.0;
    }
    return MAX(64.0, h + 10.0);
}

- (void)configureWithMessage:(NSDictionary *)msg maxWidth:(CGFloat)maxWidth {
    CTDOTheme *theme = [CTDOTheme shared];
    BOOL isSelf = [msg[@"is_self"] boolValue];
    NSString *sender = msg[@"sender_name"] ?: (isSelf ? @"Tôi" : @"Đối tác");
    NSString *content = msg[@"content"] ?: @"";
    NSString *timeLbl = msg[@"time_label"] ?: @"";
    NSString *quote = [msg[@"reply_preview"] isKindOfClass:[NSString class]] ? msg[@"reply_preview"] : nil;
    NSArray *reactions = [msg[@"reactions"] isKindOfClass:[NSArray class]] ? msg[@"reactions"] : @[];

    CGFloat bubbleMaxW = MIN(maxWidth * 0.78, 460.0);
    CGFloat textMaxW = MAX(120.0, bubbleMaxW - 24.0);
    CGRect bodyRect = [content boundingRectWithSize:CGSizeMake(textMaxW, 2000.0)
                                            options:NSStringDrawingUsesLineFragmentOrigin
                                         attributes:@{NSFontAttributeName: [UIFont systemFontOfSize:14.5]}
                                            context:nil];

    NSString *reactStr = reactions.count > 0 ? [NSString stringWithFormat:@"  %@", [reactions componentsJoinedByString:@" "]] : @"";
    NSString *metaText = [NSString stringWithFormat:@"%@%@", timeLbl, reactStr];

    CGFloat contentW = MAX(ceil(bodyRect.size.width), 130.0);
    if (quote && quote.length > 0) {
        contentW = MAX(contentW, MIN(textMaxW, 220.0));
    }
    CGFloat bubbleW = MIN(bubbleMaxW, contentW + 24.0);

    CGFloat yCursor = 8.0;
    _senderLabel.text = sender;
    _senderLabel.textColor = isSelf ? [UIColor colorWithRed:0.55 green:0.82 blue:1.0 alpha:1.0] : theme.accentOnline;
    if (theme.mode == CTDOThemeModeZaloLight && isSelf) {
        _senderLabel.textColor = theme.accentPrimary;
    }
    _senderLabel.frame = CGRectMake(12.0, yCursor, bubbleW - 24.0, 15.0);
    yCursor += 17.0;

    if (quote && quote.length > 0) {
        _quoteLabel.hidden = NO;
        _quoteLabel.text = [NSString stringWithFormat:@" ❝ %@", quote];
        _quoteLabel.textColor = theme.textSecondary;
        _quoteLabel.backgroundColor = [UIColor colorWithWhite:0.0 alpha:0.15];
        _quoteLabel.frame = CGRectMake(12.0, yCursor, bubbleW - 24.0, 22.0);
        yCursor += 26.0;
    } else {
        _quoteLabel.hidden = YES;
    }

    _bodyLabel.text = content;
    _bodyLabel.textColor = (theme.mode == CTDOThemeModeZaloLight) ? theme.textPrimary : [UIColor whiteColor];
    CGFloat bodyH = ceil(bodyRect.size.height) + 2.0;
    _bodyLabel.frame = CGRectMake(12.0, yCursor, bubbleW - 24.0, bodyH);
    yCursor += bodyH + 4.0;

    _metaLabel.text = metaText;
    _metaLabel.textColor = theme.textSecondary;
    _metaLabel.frame = CGRectMake(12.0, yCursor, bubbleW - 24.0, 14.0);
    yCursor += 18.0;

    CGFloat bubbleX = isSelf ? (maxWidth - bubbleW - 12.0) : 12.0;
    _bubbleView.frame = CGRectMake(bubbleX, 4.0, bubbleW, yCursor);
    _bubbleView.backgroundColor = isSelf ? theme.bubbleSelf : theme.bubblePartner;
    _bubbleView.layer.borderColor = theme.borderSubtle.CGColor;
}

@end

// ============================================================================
// Main View Controller Implementation
// ============================================================================
@interface CTDOMainViewController () <UITableViewDataSource, UITableViewDelegate, UITextFieldDelegate>

// Active state
@property (nonatomic, assign) NSInteger activeSlot; // 0 = Primary, 1 = Secondary Duo
@property (nonatomic, assign) NSInteger filterMode; // 0 = All, 1 = Unread, 2 = Pinned, 3 = Groups, 4 = Hidden
@property (nonatomic, copy) NSString *searchQuery;
@property (nonatomic, copy) NSString *selectedThreadIdPrimary;
@property (nonatomic, copy) NSString *selectedThreadIdSecondary; // For iPhone Duo right screen!
@property (nonatomic, assign) NSInteger secondarySlot; // Can be 0 (same account 2nd thread) or 1 (2nd account!)
@property (nonatomic, copy, nullable) NSString *replyToMsgIdPrimary;
@property (nonatomic, copy, nullable) NSString *replyPreviewTextPrimary;
@property (nonatomic, assign) BOOL compactShowingChatDetail;

@property (nonatomic, strong) NSArray<NSDictionary *> *threadsPrimary;
@property (nonatomic, strong) NSArray<NSDictionary *> *messagesPrimary;
@property (nonatomic, strong) NSArray<NSDictionary *> *messagesSecondary;
@property (nonatomic, strong, nullable) NSDictionary *githubReleaseInfo;
@property (nonatomic, assign) BOOL updateBannerDismissed;

// Top Navigation & Toolbar
@property (nonatomic, strong) UIView *topBar;
@property (nonatomic, strong) UIButton *accountButton;
@property (nonatomic, strong) UISegmentedControl *layoutModeControl;
@property (nonatomic, strong) UIButton *qrButton;
@property (nonatomic, strong) UIButton *scheduleButton;
@property (nonatomic, strong) UIButton *autoReplyButton;
@property (nonatomic, strong) UIButton *themeButton;
@property (nonatomic, strong) UIButton *updateCheckButton;

// GitHub Update Banner
@property (nonatomic, strong) UIView *updateBanner;
@property (nonatomic, strong) UILabel *updateBannerLabel;
@property (nonatomic, strong) UIButton *updateBannerDetailBtn;
@property (nonatomic, strong) UIButton *updateBannerCloseBtn;

// Workspace Containers
@property (nonatomic, strong) UIView *leftPane;    // Thread List (or Left Screen on iPhone Duo)
@property (nonatomic, strong) UIView *centerPane;  // Primary Chat
@property (nonatomic, strong) UIView *hingeBar;    // iPhone Duo Central Hinge / Mode Divider
@property (nonatomic, strong) UILabel *hingeLabel;
@property (nonatomic, strong) UIButton *hingeToggleBtn;
@property (nonatomic, strong) UIView *rightPane;   // Secondary Chat (iPhone Duo) / Inspector (iPad)

// Left Pane Views
@property (nonatomic, strong) UITextField *searchField;
@property (nonatomic, strong) UISegmentedControl *filterControl;
@property (nonatomic, strong) UITableView *threadsTableView;

// Primary Chat Views
@property (nonatomic, strong) UIView *chatHeaderPrimary;
@property (nonatomic, strong) UIButton *backButtonCompact;
@property (nonatomic, strong) UILabel *chatTitlePrimary;
@property (nonatomic, strong) UILabel *chatSubPrimary;
@property (nonatomic, strong) UIButton *simIncomingBtn;
@property (nonatomic, strong) UIButton *callBtnPrimary;
@property (nonatomic, strong) UIButton *duoSendToRightBtn;
@property (nonatomic, strong) UITableView *messagesTablePrimary;
@property (nonatomic, strong) UIView *replyBarPrimary;
@property (nonatomic, strong) UILabel *replyBarLabelPrimary;
@property (nonatomic, strong) UIButton *replyBarCancelPrimary;
@property (nonatomic, strong) UIView *composerPrimary;
@property (nonatomic, strong) UIButton *stickerBtnPrimary;
@property (nonatomic, strong) UIButton *schedQuickBtnPrimary;
@property (nonatomic, strong) UITextField *inputFieldPrimary;
@property (nonatomic, strong) UIButton *sendBtnPrimary;

// Secondary Chat Views (iPhone Duo Right Screen & iPad 3rd Column)
@property (nonatomic, strong) UIView *chatHeaderSecondary;
@property (nonatomic, strong) UILabel *chatTitleSecondary;
@property (nonatomic, strong) UILabel *chatSubSecondary;
@property (nonatomic, strong) UIButton *switchSecondaryThreadBtn;
@property (nonatomic, strong) UIButton *switchSecondarySlotBtn;
@property (nonatomic, strong) UITableView *messagesTableSecondary;
@property (nonatomic, strong) UIView *composerSecondary;
@property (nonatomic, strong) UITextField *inputFieldSecondary;
@property (nonatomic, strong) UIButton *sendBtnSecondary;

// Background Scheduler Timer
@property (nonatomic, strong, nullable) NSTimer *schedulerTimer;

@end

@implementation CTDOMainViewController

- (void)viewDidLoad {
    [super viewDidLoad];
    self.activeSlot = 0;
    self.secondarySlot = 0;
    self.filterMode = 0;
    self.searchQuery = @"";
    self.selectedThreadIdPrimary = @"uid_10001_an_kythuat";
    self.selectedThreadIdSecondary = @"uid_20002_an_kinhdoanh"; // Default iPhone Duo 2nd screen shows 2nd "Nguyễn Văn An" (#63)!
    self.preferredLayoutMode = CTDOLayoutModeAuto;
    self.duoDualChatMode = NO;

    [self buildUI];

    NSDictionary *env = [[NSProcessInfo processInfo] environment];
    NSString *initMode = env[@"CTDOZ_INITIAL_MODE"] ?: @"";
    for (NSString *arg in [[NSProcessInfo processInfo] arguments]) {
        if ([arg hasPrefix:@"--mode="]) {
            initMode = [arg substringFromIndex:7];
        }
    }
    if ([initMode isEqualToString:@"duo"]) {
        self.preferredLayoutMode = CTDOLayoutModeiPhoneDuo;
        self.duoDualChatMode = YES;
        self.layoutModeControl.selectedSegmentIndex = 2;
    } else if ([initMode isEqualToString:@"duo-list"]) {
        self.preferredLayoutMode = CTDOLayoutModeiPhoneDuo;
        self.duoDualChatMode = NO;
        self.layoutModeControl.selectedSegmentIndex = 2;
    } else if ([initMode isEqualToString:@"ipad"]) {
        self.preferredLayoutMode = CTDOLayoutModeiPadThreeColumn;
        self.layoutModeControl.selectedSegmentIndex = 3;
    } else if ([initMode isEqualToString:@"chat"]) {
        self.preferredLayoutMode = CTDOLayoutModeCompactPhone;
        self.compactShowingChatDetail = YES;
        self.layoutModeControl.selectedSegmentIndex = 1;
    } else if ([initMode isEqualToString:@"compact"]) {
        self.preferredLayoutMode = CTDOLayoutModeCompactPhone;
        self.compactShowingChatDetail = NO;
        self.layoutModeControl.selectedSegmentIndex = 1;
    }

    [self applyTheme];
    [self refreshAllData];

    // Background 2-second scheduler dispatcher (#38, #24)
    self.schedulerTimer = [NSTimer scheduledTimerWithTimeInterval:2.0
                                                           target:self
                                                         selector:@selector(onSchedulerTick)
                                                         userInfo:nil
                                                          repeats:YES];

    // Automatic GitHub Releases API check on startup
    [self triggerGitHubUpdateCheck:NO];
}

- (UIStatusBarStyle)preferredStatusBarStyle {
    return ([CTDOTheme shared].mode == CTDOThemeModeZaloLight)
        ? UIStatusBarStyleDefault
        : UIStatusBarStyleLightContent;
}

- (void)buildUI {
    // 1. Top Bar
    _topBar = [[UIView alloc] init];
    [self.view addSubview:_topBar];

    _accountButton = [UIButton buttonWithType:UIButtonTypeCustom];
    _accountButton.titleLabel.font = [UIFont boldSystemFontOfSize:12.5];
    _accountButton.titleLabel.adjustsFontSizeToFitWidth = YES;
    _accountButton.titleLabel.minimumScaleFactor = 0.8;
    _accountButton.layer.cornerRadius = 8.0;
    _accountButton.contentEdgeInsets = UIEdgeInsetsMake(4, 8, 4, 8);
    [_accountButton addTarget:self action:@selector(onTapAccountSwitch) forControlEvents:UIControlEventTouchUpInside];
    [_topBar addSubview:_accountButton];

    _layoutModeControl = [[UISegmentedControl alloc] initWithItems:@[@"Tự động", @"iPhone", @"Duo 2-Màn", @"iPad 3-Cột"]];
    _layoutModeControl.selectedSegmentIndex = 0;
    _layoutModeControl.apportionsSegmentWidthsByContent = YES;
    [_layoutModeControl addTarget:self action:@selector(onLayoutModeChanged:) forControlEvents:UIControlEventValueChanged];
    [_topBar addSubview:_layoutModeControl];

    _qrButton = [self makeTopActionButton:@"📱 QR" action:@selector(onTapQRModal)];
    _scheduleButton = [self makeTopActionButton:@"🕒 Hẹn giờ" action:@selector(onTapScheduleModal)];
    _autoReplyButton = [self makeTopActionButton:@"🤖 Bot" action:@selector(onTapAutoReplyModal)];
    _themeButton = [self makeTopActionButton:@"🎨 Giao diện" action:@selector(onTapThemeCycle)];
    _updateCheckButton = [self makeTopActionButton:@"🚀 v0.0.5" action:@selector(onTapUpdateModal)];

    [_topBar addSubview:_qrButton];
    [_topBar addSubview:_scheduleButton];
    [_topBar addSubview:_autoReplyButton];
    [_topBar addSubview:_themeButton];
    [_topBar addSubview:_updateCheckButton];

    // 2. GitHub Releases API Update Banner
    _updateBanner = [[UIView alloc] init];
    _updateBanner.hidden = YES;
    [self.view addSubview:_updateBanner];

    _updateBannerLabel = [[UILabel alloc] init];
    _updateBannerLabel.font = [UIFont boldSystemFontOfSize:12.5];
    [_updateBanner addSubview:_updateBannerLabel];

    _updateBannerDetailBtn = [UIButton buttonWithType:UIButtonTypeCustom];
    [_updateBannerDetailBtn setTitle:@"Xem chi tiết Fix & Update" forState:UIControlStateNormal];
    _updateBannerDetailBtn.titleLabel.font = [UIFont boldSystemFontOfSize:12.0];
    _updateBannerDetailBtn.layer.cornerRadius = 6.0;
    [_updateBannerDetailBtn addTarget:self action:@selector(onTapUpdateModal) forControlEvents:UIControlEventTouchUpInside];
    [_updateBanner addSubview:_updateBannerDetailBtn];

    _updateBannerCloseBtn = [UIButton buttonWithType:UIButtonTypeCustom];
    [_updateBannerCloseBtn setTitle:@"✕" forState:UIControlStateNormal];
    _updateBannerCloseBtn.titleLabel.font = [UIFont boldSystemFontOfSize:13.0];
    [_updateBannerCloseBtn addTarget:self action:@selector(onDismissUpdateBanner) forControlEvents:UIControlEventTouchUpInside];
    [_updateBanner addSubview:_updateBannerCloseBtn];

    // 3. Workspace Panes
    _leftPane = [[UIView alloc] init];
    _centerPane = [[UIView alloc] init];
    _hingeBar = [[UIView alloc] init];
    _rightPane = [[UIView alloc] init];
    [self.view addSubview:_leftPane];
    [self.view addSubview:_centerPane];
    [self.view addSubview:_hingeBar];
    [self.view addSubview:_rightPane];

    // Hinge Bar (for iPhone Duo Foldable / Dual-Screen mode)
    _hingeLabel = [[UILabel alloc] init];
    _hingeLabel.font = [UIFont boldSystemFontOfSize:10.0];
    _hingeLabel.textAlignment = NSTextAlignmentCenter;
    _hingeLabel.text = @"•\nD\nU\nO\n•";
    _hingeLabel.numberOfLines = 5;
    [_hingeBar addSubview:_hingeLabel];

    _hingeToggleBtn = [UIButton buttonWithType:UIButtonTypeCustom];
    [_hingeToggleBtn setTitle:@"⇄" forState:UIControlStateNormal];
    _hingeToggleBtn.titleLabel.font = [UIFont boldSystemFontOfSize:14.0];
    _hingeToggleBtn.layer.cornerRadius = 10.0;
    [_hingeToggleBtn addTarget:self action:@selector(onToggleDuoDualChatMode) forControlEvents:UIControlEventTouchUpInside];
    [_hingeBar addSubview:_hingeToggleBtn];

    // 4. Left Pane (Search + Filter + Threads Table)
    _searchField = [[UITextField alloc] init];
    _searchField.placeholder = @"🔍 Tìm tên Zalo, ID (#63), tin nhắn...";
    _searchField.font = [UIFont systemFontOfSize:13.5];
    _searchField.layer.cornerRadius = 8.0;
    _searchField.leftView = [[UIView alloc] initWithFrame:CGRectMake(0, 0, 10, 10)];
    _searchField.leftViewMode = UITextFieldViewModeAlways;
    _searchField.clearButtonMode = UITextFieldViewModeWhileEditing;
    _searchField.returnKeyType = UIReturnKeySearch;
    _searchField.delegate = self;
    [_searchField addTarget:self action:@selector(onSearchTextChanged:) forControlEvents:UIControlEventEditingChanged];
    [_leftPane addSubview:_searchField];

    _filterControl = [[UISegmentedControl alloc] initWithItems:@[@"Tất cả", @"Chưa đọc", @"Đã ghim", @"Nhóm", @"Đã ẩn"]];
    _filterControl.selectedSegmentIndex = 0;
    [_filterControl addTarget:self action:@selector(onFilterChanged:) forControlEvents:UIControlEventValueChanged];
    [_leftPane addSubview:_filterControl];

    _threadsTableView = [[UITableView alloc] initWithFrame:CGRectZero style:UITableViewStylePlain];
    _threadsTableView.dataSource = self;
    _threadsTableView.delegate = self;
    _threadsTableView.rowHeight = 72.0;
    _threadsTableView.separatorStyle = UITableViewCellSeparatorStyleNone;
    [_threadsTableView registerClass:[CTDOThreadCell class] forCellReuseIdentifier:@"ThreadCell"];
    UILongPressGestureRecognizer *lp = [[UILongPressGestureRecognizer alloc] initWithTarget:self action:@selector(onLongPressThread:)];
    [_threadsTableView addGestureRecognizer:lp];
    [_leftPane addSubview:_threadsTableView];

    // 5. Primary Chat Pane
    _chatHeaderPrimary = [[UIView alloc] init];
    [_centerPane addSubview:_chatHeaderPrimary];

    _backButtonCompact = [UIButton buttonWithType:UIButtonTypeCustom];
    [_backButtonCompact setTitle:@"‹ Danh sách" forState:UIControlStateNormal];
    _backButtonCompact.titleLabel.font = [UIFont boldSystemFontOfSize:14.0];
    [_backButtonCompact addTarget:self action:@selector(onTapCompactBack) forControlEvents:UIControlEventTouchUpInside];
    [_chatHeaderPrimary addSubview:_backButtonCompact];

    _chatTitlePrimary = [[UILabel alloc] init];
    _chatTitlePrimary.font = [UIFont boldSystemFontOfSize:15.5];
    [_chatHeaderPrimary addSubview:_chatTitlePrimary];

    _chatSubPrimary = [[UILabel alloc] init];
    _chatSubPrimary.font = [UIFont systemFontOfSize:11.5];
    [_chatHeaderPrimary addSubview:_chatSubPrimary];

    _simIncomingBtn = [self makeSmallHeaderButton:@"📩 Tin đến" action:@selector(onTapSimulateIncoming)];
    _callBtnPrimary = [self makeSmallHeaderButton:@"📞 Gọi" action:@selector(onTapVoiceCall)];
    _duoSendToRightBtn = [self makeSmallHeaderButton:@"⫸ Màn #2" action:@selector(onTapOpenInRightScreen)];
    [_chatHeaderPrimary addSubview:_simIncomingBtn];
    [_chatHeaderPrimary addSubview:_callBtnPrimary];
    [_chatHeaderPrimary addSubview:_duoSendToRightBtn];

    _messagesTablePrimary = [[UITableView alloc] initWithFrame:CGRectZero style:UITableViewStylePlain];
    _messagesTablePrimary.dataSource = self;
    _messagesTablePrimary.delegate = self;
    _messagesTablePrimary.separatorStyle = UITableViewCellSeparatorStyleNone;
    [_messagesTablePrimary registerClass:[CTDOMessageCell class] forCellReuseIdentifier:@"MsgCellPrimary"];
    [_centerPane addSubview:_messagesTablePrimary];

    _replyBarPrimary = [[UIView alloc] init];
    _replyBarPrimary.hidden = YES;
    [_centerPane addSubview:_replyBarPrimary];

    _replyBarLabelPrimary = [[UILabel alloc] init];
    _replyBarLabelPrimary.font = [UIFont italicSystemFontOfSize:12.0];
    [_replyBarPrimary addSubview:_replyBarLabelPrimary];

    _replyBarCancelPrimary = [UIButton buttonWithType:UIButtonTypeCustom];
    [_replyBarCancelPrimary setTitle:@"✕" forState:UIControlStateNormal];
    [_replyBarCancelPrimary addTarget:self action:@selector(onCancelReplyPrimary) forControlEvents:UIControlEventTouchUpInside];
    [_replyBarPrimary addSubview:_replyBarCancelPrimary];

    _composerPrimary = [[UIView alloc] init];
    [_centerPane addSubview:_composerPrimary];

    _stickerBtnPrimary = [self makeSmallHeaderButton:@"🎉" action:@selector(onTapStickerPicker)];
    _schedQuickBtnPrimary = [self makeSmallHeaderButton:@"🕒" action:@selector(onTapQuickSchedulePrimary)];
    [_composerPrimary addSubview:_stickerBtnPrimary];
    [_composerPrimary addSubview:_schedQuickBtnPrimary];

    _inputFieldPrimary = [[UITextField alloc] init];
    _inputFieldPrimary.placeholder = @"Nhập tin nhắn Zalo (Enter để gửi)...";
    _inputFieldPrimary.font = [UIFont systemFontOfSize:14.0];
    _inputFieldPrimary.layer.cornerRadius = 17.0;
    _inputFieldPrimary.leftView = [[UIView alloc] initWithFrame:CGRectMake(0, 0, 12, 10)];
    _inputFieldPrimary.leftViewMode = UITextFieldViewModeAlways;
    _inputFieldPrimary.returnKeyType = UIReturnKeySend;
    _inputFieldPrimary.delegate = self;
    [_composerPrimary addSubview:_inputFieldPrimary];

    _sendBtnPrimary = [UIButton buttonWithType:UIButtonTypeCustom];
    [_sendBtnPrimary setTitle:@"Gửi" forState:UIControlStateNormal];
    _sendBtnPrimary.titleLabel.font = [UIFont boldSystemFontOfSize:13.5];
    _sendBtnPrimary.layer.cornerRadius = 17.0;
    [_sendBtnPrimary addTarget:self action:@selector(onSendPrimary) forControlEvents:UIControlEventTouchUpInside];
    [_composerPrimary addSubview:_sendBtnPrimary];

    // 6. Secondary Chat Pane (iPhone Duo Right Screen & iPad 3rd Column)
    _chatHeaderSecondary = [[UIView alloc] init];
    [_rightPane addSubview:_chatHeaderSecondary];

    _chatTitleSecondary = [[UILabel alloc] init];
    _chatTitleSecondary.font = [UIFont boldSystemFontOfSize:14.5];
    [_chatHeaderSecondary addSubview:_chatTitleSecondary];

    _chatSubSecondary = [[UILabel alloc] init];
    _chatSubSecondary.font = [UIFont systemFontOfSize:11.0];
    [_chatHeaderSecondary addSubview:_chatSubSecondary];

    _switchSecondaryThreadBtn = [self makeSmallHeaderButton:@"🔄 Đổi Chat" action:@selector(onTapSwitchSecondaryThread)];
    _switchSecondarySlotBtn = [self makeSmallHeaderButton:@"👥 TK1/2" action:@selector(onTapToggleSecondarySlot)];
    [_chatHeaderSecondary addSubview:_switchSecondaryThreadBtn];
    [_chatHeaderSecondary addSubview:_switchSecondarySlotBtn];

    _messagesTableSecondary = [[UITableView alloc] initWithFrame:CGRectZero style:UITableViewStylePlain];
    _messagesTableSecondary.dataSource = self;
    _messagesTableSecondary.delegate = self;
    _messagesTableSecondary.separatorStyle = UITableViewCellSeparatorStyleNone;
    [_messagesTableSecondary registerClass:[CTDOMessageCell class] forCellReuseIdentifier:@"MsgCellSecondary"];
    [_rightPane addSubview:_messagesTableSecondary];

    _composerSecondary = [[UIView alloc] init];
    [_rightPane addSubview:_composerSecondary];

    _inputFieldSecondary = [[UITextField alloc] init];
    _inputFieldSecondary.placeholder = @"Nhắn ở màn hình Phải (iPhone Duo)...";
    _inputFieldSecondary.font = [UIFont systemFontOfSize:13.5];
    _inputFieldSecondary.layer.cornerRadius = 16.0;
    _inputFieldSecondary.leftView = [[UIView alloc] initWithFrame:CGRectMake(0, 0, 10, 10)];
    _inputFieldSecondary.leftViewMode = UITextFieldViewModeAlways;
    _inputFieldSecondary.returnKeyType = UIReturnKeySend;
    _inputFieldSecondary.delegate = self;
    [_composerSecondary addSubview:_inputFieldSecondary];

    _sendBtnSecondary = [UIButton buttonWithType:UIButtonTypeCustom];
    [_sendBtnSecondary setTitle:@"Gửi #2" forState:UIControlStateNormal];
    _sendBtnSecondary.titleLabel.font = [UIFont boldSystemFontOfSize:13.0];
    _sendBtnSecondary.layer.cornerRadius = 16.0;
    [_sendBtnSecondary addTarget:self action:@selector(onSendSecondary) forControlEvents:UIControlEventTouchUpInside];
    [_composerSecondary addSubview:_sendBtnSecondary];
}

- (UIButton *)makeTopActionButton:(NSString *)title action:(SEL)action {
    UIButton *btn = [UIButton buttonWithType:UIButtonTypeCustom];
    [btn setTitle:title forState:UIControlStateNormal];
    btn.titleLabel.font = [UIFont systemFontOfSize:11.5 weight:UIFontWeightSemibold];
    btn.titleLabel.adjustsFontSizeToFitWidth = YES;
    btn.titleLabel.minimumScaleFactor = 0.75;
    btn.layer.cornerRadius = 7.0;
    btn.contentEdgeInsets = UIEdgeInsetsMake(4, 6, 4, 6);
    [btn addTarget:self action:action forControlEvents:UIControlEventTouchUpInside];
    return btn;
}

- (UIButton *)makeSmallHeaderButton:(NSString *)title action:(SEL)action {
    UIButton *btn = [UIButton buttonWithType:UIButtonTypeCustom];
    [btn setTitle:title forState:UIControlStateNormal];
    btn.titleLabel.font = [UIFont systemFontOfSize:11.5 weight:UIFontWeightSemibold];
    btn.titleLabel.adjustsFontSizeToFitWidth = YES;
    btn.titleLabel.minimumScaleFactor = 0.75;
    btn.layer.cornerRadius = 6.0;
    btn.contentEdgeInsets = UIEdgeInsetsMake(4, 6, 4, 6);
    [btn addTarget:self action:action forControlEvents:UIControlEventTouchUpInside];
    return btn;
}

// ============================================================================
// Responsive Layout Engine for iPhone (iOS 12+), iPhone Duo, and iPad
// ============================================================================
- (CTDOLayoutMode)effectiveLayoutModeForSize:(CGSize)size {
    if (self.preferredLayoutMode != CTDOLayoutModeAuto) {
        return self.preferredLayoutMode;
    }
    if ([[CTDOCoreBridge shared] isiPhoneDuoHardwareOrGeometry:size]) {
        // On iPhone Duo folded cover (466x678), use Compact; on unfolded (951x669), use iPhone Duo Dual-Screen!
        if (size.width >= 640.0) {
            return CTDOLayoutModeiPhoneDuo;
        }
        return CTDOLayoutModeCompactPhone;
    }
    if (size.width >= 900.0) {
        return CTDOLayoutModeiPadThreeColumn;
    }
    if (size.width >= 620.0) {
        return CTDOLayoutModeiPhoneDuo;
    }
    return CTDOLayoutModeCompactPhone;
}

- (void)viewWillTransitionToSize:(CGSize)size withTransitionCoordinator:(id<UIViewControllerTransitionCoordinator>)coordinator {
    [super viewWillTransitionToSize:size withTransitionCoordinator:coordinator];
    [coordinator animateAlongsideTransition:^(id<UIViewControllerTransitionCoordinatorContext>  _Nonnull context) {
        [self.view setNeedsLayout];
        [self.view layoutIfNeeded];
    } completion:nil];
}

- (void)viewDidLayoutSubviews {
    [super viewDidLayoutSubviews];

    UIEdgeInsets safe = UIEdgeInsetsZero;
    if (@available(iOS 11.0, *)) {
        safe = self.view.safeAreaInsets;
    }
    CGFloat fullW = self.view.bounds.size.width;
    CGFloat fullH = self.view.bounds.size.height;
    CGFloat topY = safe.top;
    CGFloat bottomH = safe.bottom;

    // Top bar layout (supports compact 2-row header on narrow iPhones & 1-row on wide iPhone Duo / iPad)
    BOOL isNarrow = (fullW < 680.0);
    CGFloat topBarH = isNarrow ? 78.0 : 46.0;
    _topBar.frame = CGRectMake(0, topY, fullW, topBarH);

    if (isNarrow) {
        _accountButton.frame = CGRectMake(10, 6, 118, 30);
        _layoutModeControl.frame = CGRectMake(134, 6, fullW - 144, 30);
        CGFloat btnW = (fullW - 20 - 16) / 5.0;
        NSArray<UIButton *> *btns = @[_qrButton, _scheduleButton, _autoReplyButton, _themeButton, _updateCheckButton];
        for (NSUInteger i = 0; i < btns.count; i++) {
            btns[i].frame = CGRectMake(10 + i * (btnW + 4.0), 42, btnW, 30);
        }
    } else {
        _accountButton.frame = CGRectMake(12, 8, 148, 30);
        _layoutModeControl.frame = CGRectMake(168, 8, 305, 30);
        CGFloat rightX = fullW - 12;
        NSArray<UIButton *> *btns = @[_updateCheckButton, _themeButton, _autoReplyButton, _scheduleButton, _qrButton];
        for (UIButton *b in btns) {
            CGFloat bw = [b.titleLabel.text sizeWithAttributes:@{NSFontAttributeName: b.titleLabel.font}].width + 20.0;
            rightX -= bw;
            b.frame = CGRectMake(rightX, 8, bw, 30);
            rightX -= 6.0;
        }
    }

    CGFloat contentY = topY + topBarH;
    BOOL showBanner = (!_updateBanner.hidden && !_updateBannerDismissed);
    if (showBanner) {
        _updateBanner.frame = CGRectMake(0, contentY, fullW, 34.0);
        _updateBannerCloseBtn.frame = CGRectMake(fullW - 34, 2, 28, 30);
        _updateBannerDetailBtn.frame = CGRectMake(fullW - 196, 4, 156, 26);
        _updateBannerLabel.frame = CGRectMake(12, 2, MAX(100, fullW - 216), 30);
        contentY += 34.0;
    }

    CGFloat workspaceH = MAX(100.0, fullH - contentY - bottomH);
    CTDOLayoutMode mode = [self effectiveLayoutModeForSize:self.view.bounds.size];

    if (mode == CTDOLayoutModeCompactPhone) {
        _hingeBar.hidden = YES;
        _rightPane.hidden = YES;
        _backButtonCompact.hidden = NO;
        _duoSendToRightBtn.hidden = YES;
        if (self.compactShowingChatDetail) {
            _leftPane.hidden = YES;
            _centerPane.hidden = NO;
            _centerPane.frame = CGRectMake(0, contentY, fullW, workspaceH);
        } else {
            _leftPane.hidden = NO;
            _centerPane.hidden = YES;
            _leftPane.frame = CGRectMake(0, contentY, fullW, workspaceH);
        }
    } else if (mode == CTDOLayoutModeiPhoneDuo) {
        // iPhone Duo Dual-Screen Foldable Mode (Left Screen + 16pt Hinge + Right Screen)
        _backButtonCompact.hidden = !self.duoDualChatMode;
        _duoSendToRightBtn.hidden = NO;
        _hingeBar.hidden = NO;

        CGFloat hingeW = 18.0;
        CGFloat halfW = floor((fullW - hingeW) * 0.5);
        _hingeBar.frame = CGRectMake(halfW, contentY, hingeW, workspaceH);
        _hingeToggleBtn.frame = CGRectMake(1, 10, hingeW - 2, 28);
        _hingeLabel.frame = CGRectMake(0, 44, hingeW, 90);

        if (self.duoDualChatMode) {
            // Dual-Chat Mode: Left Screen = Chat #1, Right Screen = Chat #2!
            _leftPane.hidden = YES;
            _centerPane.hidden = NO;
            _rightPane.hidden = NO;
            _centerPane.frame = CGRectMake(0, contentY, halfW, workspaceH);
            _rightPane.frame = CGRectMake(halfW + hingeW, contentY, fullW - halfW - hingeW, workspaceH);
        } else {
            // Master-Detail Fold Mode: Left Screen = Thread List, Right Screen = Primary Chat!
            _leftPane.hidden = NO;
            _centerPane.hidden = NO;
            _rightPane.hidden = YES;
            _leftPane.frame = CGRectMake(0, contentY, halfW, workspaceH);
            _centerPane.frame = CGRectMake(halfW + hingeW, contentY, fullW - halfW - hingeW, workspaceH);
        }
    } else {
        // iPad 3-Column Mode: Thread List (275pt) + Primary Chat + Secondary Chat / Inspector (300pt)
        _backButtonCompact.hidden = YES;
        _duoSendToRightBtn.hidden = NO;
        _leftPane.hidden = NO;
        _centerPane.hidden = NO;
        _hingeBar.hidden = NO;
        _rightPane.hidden = NO;

        CGFloat leftW = 275.0;
        CGFloat hingeW = 14.0;
        CGFloat rightW = MIN(320.0, floor(fullW * 0.31));
        CGFloat centerW = MAX(260.0, fullW - leftW - hingeW - rightW);

        _leftPane.frame = CGRectMake(0, contentY, leftW, workspaceH);
        _centerPane.frame = CGRectMake(leftW, contentY, centerW, workspaceH);
        _hingeBar.frame = CGRectMake(leftW + centerW, contentY, hingeW, workspaceH);
        _hingeToggleBtn.frame = CGRectMake(0, 10, hingeW, 28);
        _hingeLabel.frame = CGRectMake(0, 44, hingeW, 90);
        _rightPane.frame = CGRectMake(leftW + centerW + hingeW, contentY, rightW, workspaceH);
    }

    [self layoutLeftPane];
    [self layoutPrimaryChatPane];
    [self layoutSecondaryChatPane];
}

- (void)layoutLeftPane {
    if (_leftPane.hidden) return;
    CGFloat w = _leftPane.bounds.size.width;
    CGFloat h = _leftPane.bounds.size.height;
    _searchField.frame = CGRectMake(10, 8, w - 20, 34);
    _filterControl.frame = CGRectMake(10, 48, w - 20, 28);
    _threadsTableView.frame = CGRectMake(0, 82, w, MAX(40, h - 82));
}

- (void)layoutPrimaryChatPane {
    if (_centerPane.hidden) return;
    CGFloat w = _centerPane.bounds.size.width;
    CGFloat h = _centerPane.bounds.size.height;

    _chatHeaderPrimary.frame = CGRectMake(0, 0, w, 54);
    CGFloat titleX = 12.0;
    if (!_backButtonCompact.hidden) {
        _backButtonCompact.frame = CGRectMake(8, 12, 82, 30);
        titleX = 94.0;
    }

    CGFloat btnRight = w - 8.0;
    if (!_duoSendToRightBtn.hidden && w >= 420.0) {
        _duoSendToRightBtn.frame = CGRectMake(btnRight - 96, 13, 96, 28);
        btnRight -= 102.0;
    } else {
        _duoSendToRightBtn.frame = CGRectZero;
    }
    _callBtnPrimary.frame = CGRectMake(btnRight - 56, 13, 56, 28);
    btnRight -= 62.0;
    _simIncomingBtn.frame = CGRectMake(btnRight - 96, 13, 96, 28);
    btnRight -= 102.0;

    CGFloat availTitleW = MAX(80.0, btnRight - titleX);
    _chatTitlePrimary.frame = CGRectMake(titleX, 8, availTitleW, 20);
    _chatSubPrimary.frame = CGRectMake(titleX, 29, availTitleW, 16);

    CGFloat composerH = 50.0;
    CGFloat replyH = _replyBarPrimary.hidden ? 0.0 : 28.0;
    _composerPrimary.frame = CGRectMake(0, h - composerH, w, composerH);
    if (replyH > 0.0) {
        _replyBarPrimary.frame = CGRectMake(0, h - composerH - replyH, w, replyH);
        _replyBarLabelPrimary.frame = CGRectMake(12, 4, w - 44, 20);
        _replyBarCancelPrimary.frame = CGRectMake(w - 32, 2, 26, 24);
    }
    _messagesTablePrimary.frame = CGRectMake(0, 54, w, MAX(40, h - 54 - composerH - replyH));

    _stickerBtnPrimary.frame = CGRectMake(8, 8, 34, 34);
    _schedQuickBtnPrimary.frame = CGRectMake(46, 8, 34, 34);
    _sendBtnPrimary.frame = CGRectMake(w - 64, 8, 56, 34);
    _inputFieldPrimary.frame = CGRectMake(86, 8, MAX(80, w - 156), 34);
}

- (void)layoutSecondaryChatPane {
    if (_rightPane.hidden) return;
    CGFloat w = _rightPane.bounds.size.width;
    CGFloat h = _rightPane.bounds.size.height;

    _chatHeaderSecondary.frame = CGRectMake(0, 0, w, 54);
    _switchSecondarySlotBtn.frame = CGRectMake(w - 78, 13, 70, 28);
    _switchSecondaryThreadBtn.frame = CGRectMake(w - 172, 13, 88, 28);
    _chatTitleSecondary.frame = CGRectMake(10, 8, MAX(70, w - 180), 20);
    _chatSubSecondary.frame = CGRectMake(10, 29, MAX(70, w - 180), 16);

    CGFloat composerH = 50.0;
    _composerSecondary.frame = CGRectMake(0, h - composerH, w, composerH);
    _messagesTableSecondary.frame = CGRectMake(0, 54, w, MAX(40, h - 54 - composerH));

    _sendBtnSecondary.frame = CGRectMake(w - 66, 8, 58, 34);
    _inputFieldSecondary.frame = CGRectMake(10, 8, MAX(70, w - 82), 34);
}

// ============================================================================
// Theme Styling
// ============================================================================
- (void)applyTheme {
    CTDOTheme *theme = [CTDOTheme shared];
    self.view.backgroundColor = theme.bgPrimary;
    _topBar.backgroundColor = theme.bgHeader;

    _accountButton.backgroundColor = theme.bgElevated;
    [_accountButton setTitleColor:theme.textPrimary forState:UIControlStateNormal];

    NSArray<UIButton *> *topBtns = @[_qrButton, _scheduleButton, _autoReplyButton, _themeButton, _updateCheckButton];
    for (UIButton *b in topBtns) {
        b.backgroundColor = theme.bgElevated;
        [b setTitleColor:theme.textPrimary forState:UIControlStateNormal];
    }
    _updateCheckButton.backgroundColor = [UIColor colorWithRed:0.06 green:0.48 blue:0.36 alpha:1.0];
    [_updateCheckButton setTitleColor:[UIColor whiteColor] forState:UIControlStateNormal];

    _updateBanner.backgroundColor = [UIColor colorWithRed:0.04 green:0.28 blue:0.22 alpha:1.0];
    _updateBannerLabel.textColor = [UIColor colorWithRed:0.45 green:0.95 blue:0.75 alpha:1.0];
    _updateBannerDetailBtn.backgroundColor = theme.accentOnline;
    [_updateBannerDetailBtn setTitleColor:[UIColor whiteColor] forState:UIControlStateNormal];
    [_updateBannerCloseBtn setTitleColor:[UIColor whiteColor] forState:UIControlStateNormal];

    _leftPane.backgroundColor = theme.bgSecondary;
    _leftPane.layer.borderColor = theme.borderSubtle.CGColor;
    _leftPane.layer.borderWidth = 0.5;

    _searchField.backgroundColor = theme.bgElevated;
    _searchField.textColor = theme.textPrimary;
    _searchField.attributedPlaceholder = [[NSAttributedString alloc]
        initWithString:@"🔍 Tìm tên Zalo, ID (#63), tin nhắn..."
            attributes:@{NSForegroundColorAttributeName: theme.textSecondary}];
    _threadsTableView.backgroundColor = theme.bgSecondary;

    NSDictionary *segNormal = @{
        NSForegroundColorAttributeName: theme.textSecondary,
        NSFontAttributeName: [UIFont systemFontOfSize:11.5 weight:UIFontWeightSemibold]
    };
    NSDictionary *segSelected = @{
        NSForegroundColorAttributeName: (theme.mode == CTDOThemeModeZaloLight) ? [UIColor blackColor] : [UIColor whiteColor],
        NSFontAttributeName: [UIFont boldSystemFontOfSize:11.5]
    };
    [_layoutModeControl setTitleTextAttributes:segNormal forState:UIControlStateNormal];
    [_layoutModeControl setTitleTextAttributes:segSelected forState:UIControlStateSelected];
    [_filterControl setTitleTextAttributes:segNormal forState:UIControlStateNormal];
    [_filterControl setTitleTextAttributes:segSelected forState:UIControlStateSelected];
    if (@available(iOS 13.0, *)) {
        _layoutModeControl.selectedSegmentTintColor = theme.accentPrimary;
        _filterControl.selectedSegmentTintColor = theme.accentPrimary;
    }

    _centerPane.backgroundColor = theme.bgPrimary;
    _chatHeaderPrimary.backgroundColor = theme.bgSecondary;
    _chatTitlePrimary.textColor = theme.textPrimary;
    _chatSubPrimary.textColor = theme.textSecondary;
    [_backButtonCompact setTitleColor:theme.accentPrimary forState:UIControlStateNormal];

    for (UIButton *b in @[_simIncomingBtn, _callBtnPrimary, _duoSendToRightBtn, _stickerBtnPrimary, _schedQuickBtnPrimary]) {
        b.backgroundColor = theme.bgElevated;
        [b setTitleColor:theme.textPrimary forState:UIControlStateNormal];
    }

    _messagesTablePrimary.backgroundColor = theme.bgPrimary;
    _replyBarPrimary.backgroundColor = theme.bgElevated;
    _replyBarLabelPrimary.textColor = theme.textSecondary;
    [_replyBarCancelPrimary setTitleColor:theme.textPrimary forState:UIControlStateNormal];

    _composerPrimary.backgroundColor = theme.bgSecondary;
    _inputFieldPrimary.backgroundColor = theme.bgElevated;
    _inputFieldPrimary.textColor = theme.textPrimary;
    _inputFieldPrimary.attributedPlaceholder = [[NSAttributedString alloc]
        initWithString:@"Nhập tin nhắn Zalo (Enter để gửi)..."
            attributes:@{NSForegroundColorAttributeName: theme.textSecondary}];
    _sendBtnPrimary.backgroundColor = theme.accentPrimary;
    [_sendBtnPrimary setTitleColor:[UIColor whiteColor] forState:UIControlStateNormal];

    _hingeBar.backgroundColor = [UIColor colorWithRed:0.02 green:0.03 blue:0.06 alpha:1.0];
    _hingeLabel.textColor = theme.accentPrimary;
    _hingeToggleBtn.backgroundColor = theme.accentPrimary;
    [_hingeToggleBtn setTitleColor:[UIColor whiteColor] forState:UIControlStateNormal];

    _rightPane.backgroundColor = theme.bgPrimary;
    _chatHeaderSecondary.backgroundColor = theme.bgSecondary;
    _chatTitleSecondary.textColor = theme.textPrimary;
    _chatSubSecondary.textColor = theme.accentOnline;
    for (UIButton *b in @[_switchSecondaryThreadBtn, _switchSecondarySlotBtn]) {
        b.backgroundColor = theme.bgElevated;
        [b setTitleColor:theme.textPrimary forState:UIControlStateNormal];
    }
    _messagesTableSecondary.backgroundColor = theme.bgPrimary;
    _composerSecondary.backgroundColor = theme.bgSecondary;
    _inputFieldSecondary.backgroundColor = theme.bgElevated;
    _inputFieldSecondary.textColor = theme.textPrimary;
    _inputFieldSecondary.attributedPlaceholder = [[NSAttributedString alloc]
        initWithString:@"Nhắn ở màn hình Phải (iPhone Duo)..."
            attributes:@{NSForegroundColorAttributeName: theme.textSecondary}];
    _sendBtnSecondary.backgroundColor = theme.accentOnline;
    [_sendBtnSecondary setTitleColor:[UIColor whiteColor] forState:UIControlStateNormal];

    [self setNeedsStatusBarAppearanceUpdate];
}

// ============================================================================
// Data Loading & Sync
// ============================================================================
- (void)refreshAllData {
    CTDOCoreBridge *core = [CTDOCoreBridge shared];
    NSDictionary *profile = [core switchAccount:self.activeSlot];
    NSString *accTitle = (self.activeSlot == 0)
        ? @"👤 TK1 · Đổi TK"
        : @"👥 TK2 · Duo #2";
    if (profile[@"display_name"]) {
        accTitle = (self.activeSlot == 0)
            ? @"👤 TK1 · Đổi TK"
            : @"👥 TK2 · Duo #2";
    }
    [_accountButton setTitle:accTitle forState:UIControlStateNormal];

    self.threadsPrimary = [core listThreadsForSlot:self.activeSlot
                                        filterMode:self.filterMode
                                       searchQuery:self.searchQuery];

    // Ensure selectedThreadIdPrimary is valid by exact thread_id (#63)
    BOOL foundPrimary = NO;
    NSDictionary *primaryThreadObj = nil;
    for (NSDictionary *t in self.threadsPrimary) {
        if ([t[@"id"] isEqualToString:self.selectedThreadIdPrimary]) {
            foundPrimary = YES;
            primaryThreadObj = t;
            break;
        }
    }
    if (!foundPrimary && self.threadsPrimary.count > 0) {
        primaryThreadObj = self.threadsPrimary.firstObject;
        self.selectedThreadIdPrimary = primaryThreadObj[@"id"] ?: @"cloud_my";
    }

    if (primaryThreadObj) {
        _chatTitlePrimary.text = primaryThreadObj[@"name"] ?: @"Hội thoại";
        _chatSubPrimary.text = [NSString stringWithFormat:@"[%@] %@",
                                primaryThreadObj[@"id"] ?: @"",
                                primaryThreadObj[@"subtitle_tag"] ?: @""];
    }

    self.messagesPrimary = [core getMessagesForSlot:self.activeSlot
                                           threadId:self.selectedThreadIdPrimary];

    // Secondary Chat (Right Screen on iPhone Duo / iPad)
    NSArray<NSDictionary *> *secThreads = [core listThreadsForSlot:self.secondarySlot
                                                        filterMode:0
                                                       searchQuery:@""];
    NSDictionary *secThreadObj = nil;
    for (NSDictionary *t in secThreads) {
        if ([t[@"id"] isEqualToString:self.selectedThreadIdSecondary]) {
            secThreadObj = t;
            break;
        }
    }
    if (!secThreadObj && secThreads.count > 0) {
        secThreadObj = (secThreads.count > 1) ? secThreads[1] : secThreads[0];
        self.selectedThreadIdSecondary = secThreadObj[@"id"] ?: @"cloud_my";
    }
    if (secThreadObj) {
        _chatTitleSecondary.text = [NSString stringWithFormat:@"📱 Màn #2 (TK%ld): %@",
                                    (long)(self.secondarySlot + 1),
                                    secThreadObj[@"name"] ?: @"Hội thoại"];
        _chatSubSecondary.text = [NSString stringWithFormat:@"ID: %@ · %@",
                                  secThreadObj[@"id"] ?: @"",
                                  secThreadObj[@"subtitle_tag"] ?: @""];
    }
    self.messagesSecondary = [core getMessagesForSlot:self.secondarySlot
                                             threadId:self.selectedThreadIdSecondary];

    [_threadsTableView reloadData];
    [_messagesTablePrimary reloadData];
    [_messagesTableSecondary reloadData];
    [self scrollTableToBottom:_messagesTablePrimary count:self.messagesPrimary.count];
    [self scrollTableToBottom:_messagesTableSecondary count:self.messagesSecondary.count];
}

- (void)scrollTableToBottom:(UITableView *)table count:(NSUInteger)count {
    if (count == 0) return;
    NSIndexPath *last = [NSIndexPath indexPathForRow:(NSInteger)(count - 1) inSection:0];
    [table scrollToRowAtIndexPath:last atScrollPosition:UITableViewScrollPositionBottom animated:NO];
}

// ============================================================================
// UITableViewDataSource & UITableViewDelegate
// ============================================================================
- (NSInteger)tableView:(UITableView *)tableView numberOfRowsInSection:(NSInteger)section {
    if (tableView == _threadsTableView) {
        return (NSInteger)self.threadsPrimary.count;
    }
    if (tableView == _messagesTablePrimary) {
        return (NSInteger)self.messagesPrimary.count;
    }
    return (NSInteger)self.messagesSecondary.count;
}

- (CGFloat)tableView:(UITableView *)tableView heightForRowAtIndexPath:(NSIndexPath *)indexPath {
    if (tableView == _threadsTableView) {
        return 72.0;
    }
    if (tableView == _messagesTablePrimary) {
        NSDictionary *msg = self.messagesPrimary[indexPath.row];
        return [CTDOMessageCell heightForMessage:msg maxWidth:MAX(260.0, tableView.bounds.size.width)];
    }
    NSDictionary *msg = self.messagesSecondary[indexPath.row];
    return [CTDOMessageCell heightForMessage:msg maxWidth:MAX(240.0, tableView.bounds.size.width)];
}

- (UITableViewCell *)tableView:(UITableView *)tableView cellForRowAtIndexPath:(NSIndexPath *)indexPath {
    if (tableView == _threadsTableView) {
        CTDOThreadCell *cell = [tableView dequeueReusableCellWithIdentifier:@"ThreadCell" forIndexPath:indexPath];
        NSDictionary *thread = self.threadsPrimary[indexPath.row];
        BOOL selected = [thread[@"id"] isEqualToString:self.selectedThreadIdPrimary];
        [cell configureWithThread:thread isSelected:selected];
        return cell;
    }
    if (tableView == _messagesTablePrimary) {
        CTDOMessageCell *cell = [tableView dequeueReusableCellWithIdentifier:@"MsgCellPrimary" forIndexPath:indexPath];
        NSDictionary *msg = self.messagesPrimary[indexPath.row];
        [cell configureWithMessage:msg maxWidth:MAX(260.0, tableView.bounds.size.width)];
        return cell;
    }
    CTDOMessageCell *cell = [tableView dequeueReusableCellWithIdentifier:@"MsgCellSecondary" forIndexPath:indexPath];
    NSDictionary *msg = self.messagesSecondary[indexPath.row];
    [cell configureWithMessage:msg maxWidth:MAX(240.0, tableView.bounds.size.width)];
    return cell;
}

- (void)tableView:(UITableView *)tableView didSelectRowAtIndexPath:(NSIndexPath *)indexPath {
    if (tableView == _threadsTableView) {
        NSDictionary *thread = self.threadsPrimary[indexPath.row];
        // Strict selection by thread["id"] (#63), never by display name!
        self.selectedThreadIdPrimary = thread[@"id"] ?: @"cloud_my";
        self.compactShowingChatDetail = YES;
        [self refreshAllData];
        [self.view setNeedsLayout];
        return;
    }

    // Message bubble tapped -> Offer Reaction or Reply Quote!
    BOOL isPrimary = (tableView == _messagesTablePrimary);
    NSArray<NSDictionary *> *list = isPrimary ? self.messagesPrimary : self.messagesSecondary;
    if (indexPath.row >= (NSInteger)list.count) return;
    NSDictionary *msg = list[indexPath.row];
    NSString *mid = msg[@"id"] ?: @"";
    NSString *tid = isPrimary ? self.selectedThreadIdPrimary : self.selectedThreadIdSecondary;
    NSInteger slot = isPrimary ? self.activeSlot : self.secondarySlot;

    UIAlertController *sheet = [UIAlertController alertControllerWithTitle:msg[@"sender_name"]
                                                                   message:msg[@"content"]
                                                            preferredStyle:UIAlertControllerStyleActionSheet];
    for (NSString *emoji in @[@"❤️", @"👍", @"😆", @"😮", @"🔥"]) {
        [sheet addAction:[UIAlertAction actionWithTitle:[NSString stringWithFormat:@"Thả biểu cảm %@", emoji]
                                                  style:UIAlertActionStyleDefault
                                                handler:^(UIAlertAction * _Nonnull action) {
            [[CTDOCoreBridge shared] reactMessageForSlot:slot threadId:tid msgId:mid emoji:emoji];
            [self refreshAllData];
        }]];
    }
    if (isPrimary) {
        [sheet addAction:[UIAlertAction actionWithTitle:@"↩️ Trả lời (Quote Reply) tin nhắn này"
                                                  style:UIAlertActionStyleDefault
                                                handler:^(UIAlertAction * _Nonnull action) {
            self.replyToMsgIdPrimary = mid;
            self.replyPreviewTextPrimary = [NSString stringWithFormat:@"Đang trả lời %@: %@", msg[@"sender_name"], msg[@"content"]];
            self.replyBarLabelPrimary.text = self.replyPreviewTextPrimary;
            self.replyBarPrimary.hidden = NO;
            [self.view setNeedsLayout];
            [self.inputFieldPrimary becomeFirstResponder];
        }]];
    }
    [sheet addAction:[UIAlertAction actionWithTitle:@"Đóng" style:UIAlertActionStyleCancel handler:nil]];
    [self presentPopoverOrSheet:sheet sourceView:tableView];
}

// iOS 12+ Swipe Actions on Threads (#5, #24)
- (UISwipeActionsConfiguration *)tableView:(UITableView *)tableView leadingSwipeActionsConfigurationForRowAtIndexPath:(NSIndexPath *)indexPath {
    if (tableView != _threadsTableView) return nil;
    NSDictionary *thread = self.threadsPrimary[indexPath.row];
    NSString *tid = thread[@"id"] ?: @"";
    BOOL pinned = [thread[@"is_pinned"] boolValue];

    UIContextualAction *pinAct = [UIContextualAction contextualActionWithStyle:UIContextualActionStyleNormal
                                                                         title:(pinned ? @"Bỏ ghim" : @"📌 Ghim")
                                                                       handler:^(UIContextualAction *action, UIView *sourceView, void (^completionHandler)(BOOL)) {
        [[CTDOCoreBridge shared] performThreadActionForSlot:self.activeSlot threadId:tid action:1];
        [self refreshAllData];
        completionHandler(YES);
    }];
    pinAct.backgroundColor = [CTDOTheme shared].accentPrimary;

    UIContextualAction *duoAct = [UIContextualAction contextualActionWithStyle:UIContextualActionStyleNormal
                                                                         title:@"📱 Màn #2"
                                                                       handler:^(UIContextualAction *action, UIView *sourceView, void (^completionHandler)(BOOL)) {
        self.secondarySlot = self.activeSlot;
        self.selectedThreadIdSecondary = tid;
        self.preferredLayoutMode = CTDOLayoutModeiPhoneDuo;
        self.duoDualChatMode = YES;
        self.layoutModeControl.selectedSegmentIndex = 2;
        [self refreshAllData];
        [self.view setNeedsLayout];
        completionHandler(YES);
    }];
    duoAct.backgroundColor = [CTDOTheme shared].accentOnline;

    return [UISwipeActionsConfiguration configurationWithActions:@[pinAct, duoAct]];
}

- (UISwipeActionsConfiguration *)tableView:(UITableView *)tableView trailingSwipeActionsConfigurationForRowAtIndexPath:(NSIndexPath *)indexPath {
    if (tableView != _threadsTableView) return nil;
    NSDictionary *thread = self.threadsPrimary[indexPath.row];
    NSString *tid = thread[@"id"] ?: @"";
    BOOL muted = [thread[@"is_muted"] boolValue];
    BOOL hidden = [thread[@"is_hidden"] boolValue];

    UIContextualAction *muteAct = [UIContextualAction contextualActionWithStyle:UIContextualActionStyleNormal
                                                                          title:(muted ? @"🔔 Bật TB" : @"🔕 Tắt TB")
                                                                        handler:^(UIContextualAction *action, UIView *sourceView, void (^completionHandler)(BOOL)) {
        [[CTDOCoreBridge shared] performThreadActionForSlot:self.activeSlot threadId:tid action:2];
        [self refreshAllData];
        completionHandler(YES);
    }];
    muteAct.backgroundColor = [CTDOTheme shared].warningColor;

    UIContextualAction *hideAct = [UIContextualAction contextualActionWithStyle:UIContextualActionStyleNormal
                                                                          title:(hidden ? @"👁 Hiện" : @"🙈 Ẩn")
                                                                        handler:^(UIContextualAction *action, UIView *sourceView, void (^completionHandler)(BOOL)) {
        [[CTDOCoreBridge shared] performThreadActionForSlot:self.activeSlot threadId:tid action:3];
        [self refreshAllData];
        completionHandler(YES);
    }];
    hideAct.backgroundColor = [UIColor darkGrayColor];

    UIContextualAction *delAct = [UIContextualAction contextualActionWithStyle:UIContextualActionStyleDestructive
                                                                         title:@"🗑 Xoá"
                                                                       handler:^(UIContextualAction *action, UIView *sourceView, void (^completionHandler)(BOOL)) {
        [[CTDOCoreBridge shared] performThreadActionForSlot:self.activeSlot threadId:tid action:4];
        [self refreshAllData];
        completionHandler(YES);
    }];

    return [UISwipeActionsConfiguration configurationWithActions:@[delAct, hideAct, muteAct]];
}

// ============================================================================
// User Actions & Modals
// ============================================================================
- (void)presentPopoverOrSheet:(UIAlertController *)alert sourceView:(UIView *)sourceView {
    UIPopoverPresentationController *pop = alert.popoverPresentationController;
    if (pop) {
        pop.sourceView = sourceView;
        pop.sourceRect = CGRectMake(sourceView.bounds.size.width * 0.5, sourceView.bounds.size.height * 0.5, 1, 1);
    }
    [self presentViewController:alert animated:YES completion:nil];
}

- (void)onLongPressThread:(UILongPressGestureRecognizer *)gesture {
    if (gesture.state != UIGestureRecognizerStateBegan) return;
    CGPoint pt = [gesture locationInView:_threadsTableView];
    NSIndexPath *idx = [_threadsTableView indexPathForRowAtPoint:pt];
    if (!idx || idx.row >= (NSInteger)self.threadsPrimary.count) return;

    NSDictionary *thread = self.threadsPrimary[idx.row];
    NSString *tid = thread[@"id"] ?: @"";
    NSString *tname = thread[@"name"] ?: @"Hội thoại";

    UIAlertController *menu = [UIAlertController alertControllerWithTitle:tname
                                                                  message:[NSString stringWithFormat:@"Thread ID: %@ (#5, #24, #63)", tid]
                                                           preferredStyle:UIAlertControllerStyleActionSheet];
    [menu addAction:[UIAlertAction actionWithTitle:@"📌 Ghim / Bỏ ghim hội thoại" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        [[CTDOCoreBridge shared] performThreadActionForSlot:self.activeSlot threadId:tid action:1];
        [self refreshAllData];
    }]];
    [menu addAction:[UIAlertAction actionWithTitle:@"🔕 Tắt / Bật thông báo" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        [[CTDOCoreBridge shared] performThreadActionForSlot:self.activeSlot threadId:tid action:2];
        [self refreshAllData];
    }]];
    [menu addAction:[UIAlertAction actionWithTitle:@"📱 Mở song song ở Màn hình Phải (iPhone Duo)" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        self.secondarySlot = self.activeSlot;
        self.selectedThreadIdSecondary = tid;
        self.preferredLayoutMode = CTDOLayoutModeiPhoneDuo;
        self.duoDualChatMode = YES;
        self.layoutModeControl.selectedSegmentIndex = 2;
        [self refreshAllData];
        [self.view setNeedsLayout];
    }]];
    [menu addAction:[UIAlertAction actionWithTitle:@"🕒 Hẹn giờ gửi tin nhắn vào hội thoại này (#38)" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        self.selectedThreadIdPrimary = tid;
        [self onTapQuickSchedulePrimary];
    }]];
    [menu addAction:[UIAlertAction actionWithTitle:@"🙈 Ẩn / Hiện hội thoại" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        [[CTDOCoreBridge shared] performThreadActionForSlot:self.activeSlot threadId:tid action:3];
        [self refreshAllData];
    }]];
    [menu addAction:[UIAlertAction actionWithTitle:@"🗑 Xoá lịch sử cuộc trò chuyện" style:UIAlertActionStyleDestructive handler:^(UIAlertAction *action) {
        [[CTDOCoreBridge shared] performThreadActionForSlot:self.activeSlot threadId:tid action:4];
        [self refreshAllData];
    }]];
    [menu addAction:[UIAlertAction actionWithTitle:@"Huỷ" style:UIAlertActionStyleCancel handler:nil]];
    [self presentPopoverOrSheet:menu sourceView:_threadsTableView];
}

- (void)onTapAccountSwitch {
    UIAlertController *sheet = [UIAlertController alertControllerWithTitle:@"Chuyển đổi Tài khoản Zalo (Dual Account)"
                                                                   message:@"Hỗ trợ chạy 2 tài khoản độc lập trên iPhone / iPad / iPhone Duo"
                                                            preferredStyle:UIAlertControllerStyleActionSheet];
    [sheet addAction:[UIAlertAction actionWithTitle:@"👤 Tài khoản 1: Đỗ Trung Thành (TK Chính)" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        self.activeSlot = 0;
        [self refreshAllData];
    }]];
    [sheet addAction:[UIAlertAction actionWithTitle:@"👥 Tài khoản 2: CTDO Team Work (TK Duo #2)" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        self.activeSlot = 1;
        [self refreshAllData];
    }]];
    [sheet addAction:[UIAlertAction actionWithTitle:@"⚡ Mở song song TK1 (Màn Trái) + TK2 (Màn Phải) trên iPhone Duo" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        self.activeSlot = 0;
        self.secondarySlot = 1;
        self.selectedThreadIdSecondary = @"uid_90001_khachhang_vip";
        self.preferredLayoutMode = CTDOLayoutModeiPhoneDuo;
        self.duoDualChatMode = YES;
        self.layoutModeControl.selectedSegmentIndex = 2;
        [self refreshAllData];
        [self.view setNeedsLayout];
    }]];
    [sheet addAction:[UIAlertAction actionWithTitle:@"📦 Xuất / Nhập (Migrate) lịch sử tin nhắn (#9)" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        [self showHistoryMigrationModal];
    }]];
    [sheet addAction:[UIAlertAction actionWithTitle:@"Đóng" style:UIAlertActionStyleCancel handler:nil]];
    [self presentPopoverOrSheet:sheet sourceView:_accountButton];
}

- (void)onLayoutModeChanged:(UISegmentedControl *)seg {
    self.preferredLayoutMode = (CTDOLayoutMode)seg.selectedSegmentIndex;
    if (self.preferredLayoutMode == CTDOLayoutModeiPhoneDuo) {
        // Default to showing Dual-Chat side-by-side when user explicitly taps "iPhone Duo"!
        self.duoDualChatMode = YES;
    }
    [self.view setNeedsLayout];
    [self refreshAllData];
}

- (void)onToggleDuoDualChatMode {
    self.duoDualChatMode = !self.duoDualChatMode;
    [self.view setNeedsLayout];
    [self refreshAllData];
}

- (void)onTapCompactBack {
    if ([self effectiveLayoutModeForSize:self.view.bounds.size] == CTDOLayoutModeiPhoneDuo && self.duoDualChatMode) {
        self.duoDualChatMode = NO;
    } else {
        self.compactShowingChatDetail = NO;
    }
    [self.view setNeedsLayout];
}

- (void)onSearchTextChanged:(UITextField *)tf {
    self.searchQuery = tf.text ?: @"";
    [self refreshAllData];
}

- (void)onFilterChanged:(UISegmentedControl *)seg {
    self.filterMode = seg.selectedSegmentIndex;
    [self refreshAllData];
}

- (BOOL)textFieldShouldReturn:(UITextField *)textField {
    if (textField == _inputFieldPrimary) {
        [self onSendPrimary];
        return NO;
    }
    if (textField == _inputFieldSecondary) {
        [self onSendSecondary];
        return NO;
    }
    [textField resignFirstResponder];
    return YES;
}

- (void)onSendPrimary {
    NSString *txt = [_inputFieldPrimary.text stringByTrimmingCharactersInSet:[NSCharacterSet whitespaceAndNewlineCharacterSet]];
    if (txt.length == 0) return;
    [[CTDOCoreBridge shared] sendMessageForSlot:self.activeSlot
                                       threadId:self.selectedThreadIdPrimary
                                        content:txt
                                   replyToMsgId:self.replyToMsgIdPrimary];
    _inputFieldPrimary.text = @"";
    [self onCancelReplyPrimary];
    [self refreshAllData];
}

- (void)onSendSecondary {
    NSString *txt = [_inputFieldSecondary.text stringByTrimmingCharactersInSet:[NSCharacterSet whitespaceAndNewlineCharacterSet]];
    if (txt.length == 0) return;
    [[CTDOCoreBridge shared] sendMessageForSlot:self.secondarySlot
                                       threadId:self.selectedThreadIdSecondary
                                        content:txt
                                   replyToMsgId:nil];
    _inputFieldSecondary.text = @"";
    [self refreshAllData];
}

- (void)onCancelReplyPrimary {
    self.replyToMsgIdPrimary = nil;
    self.replyPreviewTextPrimary = nil;
    self.replyBarPrimary.hidden = YES;
    [self.view setNeedsLayout];
}

- (void)onTapSimulateIncoming {
    UIAlertController *alert = [UIAlertController alertControllerWithTitle:@"📩 Giả lập Tin nhắn đến & Test Auto-Reply (#35)"
                                                                   message:@"Nhập nội dung đối tác gửi đến (ví dụ có chữ 'báo giá' hoặc 'bận' để kích hoạt Bot tự động trả lời):"
                                                            preferredStyle:UIAlertControllerStyleAlert];
    [alert addTextFieldWithConfigurationHandler:^(UITextField *tf) {
        tf.text = @"Chào anh, gửi giúp em báo giá hợp đồng nhé!";
    }];
    [alert addAction:[UIAlertAction actionWithTitle:@"Gửi tin đến" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        NSString *txt = alert.textFields.firstObject.text ?: @"Xin chào!";
        [[CTDOCoreBridge shared] simulateIncomingForSlot:self.activeSlot
                                                threadId:self.selectedThreadIdPrimary
                                              senderName:nil
                                                 content:txt];
        [self refreshAllData];
    }]];
    [alert addAction:[UIAlertAction actionWithTitle:@"Huỷ" style:UIAlertActionStyleCancel handler:nil]];
    [self presentViewController:alert animated:YES completion:nil];
}

- (void)onTapVoiceCall {
    NSDictionary *signedDiag = [[CTDOCoreBridge shared] buildSignedRequestForService:@"chat"
                                                                              method:@"voip_call_init"
                                                                          jsonParams:[NSString stringWithFormat:@"{\"callee\":\"%@\"}", self.selectedThreadIdPrimary]];
    NSString *msg = [NSString stringWithFormat:@"Đang kết nối VoIP Opus 48kHz tới [%@]...\n\n• Giao thức: ParamCipher v2 (zpw_ver=671)\n• Signkey: %@\n• Endpoint: %@",
                     self.selectedThreadIdPrimary,
                     signedDiag[@"signkey"] ?: @"",
                     signedDiag[@"endpoint"] ?: @""];
    UIAlertController *callAlert = [UIAlertController alertControllerWithTitle:@"📞 Cuộc gọi Thoại / Video Zalo"
                                                                       message:msg
                                                                preferredStyle:UIAlertControllerStyleAlert];
    [callAlert addAction:[UIAlertAction actionWithTitle:@"Kết thúc cuộc gọi" style:UIAlertActionStyleDestructive handler:nil]];
    [self presentViewController:callAlert animated:YES completion:nil];
}

- (void)onTapOpenInRightScreen {
    self.secondarySlot = self.activeSlot;
    self.selectedThreadIdSecondary = self.selectedThreadIdPrimary;
    self.preferredLayoutMode = CTDOLayoutModeiPhoneDuo;
    self.duoDualChatMode = YES;
    self.layoutModeControl.selectedSegmentIndex = 2;
    [self refreshAllData];
    [self.view setNeedsLayout];
}

- (void)onTapSwitchSecondaryThread {
    NSArray<NSDictionary *> *threads = [[CTDOCoreBridge shared] listThreadsForSlot:self.secondarySlot filterMode:0 searchQuery:@""];
    UIAlertController *sheet = [UIAlertController alertControllerWithTitle:@"Chọn hội thoại cho Màn hình Phải (#2)"
                                                                   message:@"Hỗ trợ so sánh trực tiếp 2 người trùng tên (#63) trên iPhone Duo"
                                                            preferredStyle:UIAlertControllerStyleActionSheet];
    for (NSDictionary *t in threads) {
        NSString *tid = t[@"id"] ?: @"";
        NSString *label = [NSString stringWithFormat:@"%@ (%@)", t[@"name"], t[@"subtitle_tag"]];
        [sheet addAction:[UIAlertAction actionWithTitle:label style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
            self.selectedThreadIdSecondary = tid;
            [self refreshAllData];
        }]];
    }
    [sheet addAction:[UIAlertAction actionWithTitle:@"Đóng" style:UIAlertActionStyleCancel handler:nil]];
    [self presentPopoverOrSheet:sheet sourceView:_switchSecondaryThreadBtn];
}

- (void)onTapToggleSecondarySlot {
    self.secondarySlot = (self.secondarySlot == 0) ? 1 : 0;
    [self refreshAllData];
}

- (void)onTapStickerPicker {
    UIAlertController *sheet = [UIAlertController alertControllerWithTitle:@"🎉 Bộ Nhãn dán (custom_stickers.json #7)"
                                                                   message:@"Chọn nhãn dán để gửi ngay"
                                                            preferredStyle:UIAlertControllerStyleActionSheet];
    NSArray<NSString *> *stickers = @[
        @"[Sticker] 🐱 Mèo Meng Meng: Xin chào nhé!",
        @"[Sticker] 🚀 Tên lửa ctdozlite: Chốt đơn 120fps!",
        @"[Sticker] ❤️ Thả tim siêu to khổng lồ!",
        @"[Sticker] ☕ Mời cà phê sáng tốt lành!"
    ];
    for (NSString *st in stickers) {
        [sheet addAction:[UIAlertAction actionWithTitle:st style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
            [[CTDOCoreBridge shared] sendMessageForSlot:self.activeSlot
                                               threadId:self.selectedThreadIdPrimary
                                                content:st
                                           replyToMsgId:self.replyToMsgIdPrimary];
            [self onCancelReplyPrimary];
            [self refreshAllData];
        }]];
    }
    [sheet addAction:[UIAlertAction actionWithTitle:@"Đóng" style:UIAlertActionStyleCancel handler:nil]];
    [self presentPopoverOrSheet:sheet sourceView:_stickerBtnPrimary];
}

- (void)onTapQuickSchedulePrimary {
    UIAlertController *alert = [UIAlertController alertControllerWithTitle:@"🕒 Hẹn giờ gửi tin nhắn (#38, #24)"
                                                                   message:[NSString stringWithFormat:@"Hội thoại đích: %@", self.selectedThreadIdPrimary]
                                                            preferredStyle:UIAlertControllerStyleAlert];
    [alert addTextFieldWithConfigurationHandler:^(UITextField *tf) {
        tf.placeholder = @"Nhập nội dung tin nhắn hẹn giờ...";
        tf.text = @"⏰ Nhắc hẹn tự động từ ctdozlite iOS!";
    }];
    int64_t nowMs = (int64_t)([[NSDate date] timeIntervalSince1970] * 1000.0);
    [alert addAction:[UIAlertAction actionWithTitle:@"⚡ Gửi sau 3 giây (Test ngay)" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        NSString *txt = alert.textFields.firstObject.text ?: @"";
        [[CTDOCoreBridge shared] scheduleMessageForSlot:self.activeSlot threadId:self.selectedThreadIdPrimary content:txt deliverAtMs:(nowMs + 3000)];
    }]];
    [alert addAction:[UIAlertAction actionWithTitle:@"🕒 Sau 15 phút (+15m)" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        NSString *txt = alert.textFields.firstObject.text ?: @"";
        [[CTDOCoreBridge shared] scheduleMessageForSlot:self.activeSlot threadId:self.selectedThreadIdPrimary content:txt deliverAtMs:(nowMs + 15 * 60 * 1000)];
    }]];
    [alert addAction:[UIAlertAction actionWithTitle:@"🕒 Sau 1 giờ (+1h)" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        NSString *txt = alert.textFields.firstObject.text ?: @"";
        [[CTDOCoreBridge shared] scheduleMessageForSlot:self.activeSlot threadId:self.selectedThreadIdPrimary content:txt deliverAtMs:(nowMs + 60 * 60 * 1000)];
    }]];
    [alert addAction:[UIAlertAction actionWithTitle:@"Huỷ" style:UIAlertActionStyleCancel handler:nil]];
    [self presentViewController:alert animated:YES completion:nil];
}

- (void)onSchedulerTick {
    NSArray<NSDictionary *> *dispatched = [[CTDOCoreBridge shared] pollDueScheduledMessages];
    if (dispatched.count > 0) {
        [self refreshAllData];
    }
}

- (void)onTapScheduleModal {
    NSArray<NSDictionary *> *items = [[CTDOCoreBridge shared] listScheduledForSlot:self.activeSlot];
    NSMutableString *desc = [NSMutableString stringWithFormat:@"Đang có %lu tin nhắn chờ gửi tự động:\n\n", (unsigned long)items.count];
    for (NSDictionary *it in items) {
        [desc appendFormat:@"• [#%@] %@ (%@): %@\n",
         it[@"id"], it[@"thread_name"], it[@"deliver_time_label"], it[@"content"]];
    }
    UIAlertController *alert = [UIAlertController alertControllerWithTitle:@"🕒 Quản lý Tin nhắn Hẹn giờ (#38, #24)"
                                                                   message:desc
                                                            preferredStyle:UIAlertControllerStyleAlert];
    [alert addAction:[UIAlertAction actionWithTitle:@"➕ Thêm tin hẹn giờ mới" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        [self onTapQuickSchedulePrimary];
    }]];
    if (items.count > 0) {
        int64_t firstId = [items.firstObject[@"id"] longLongValue];
        [alert addAction:[UIAlertAction actionWithTitle:[NSString stringWithFormat:@"🗑 Huỷ tin hẹn giờ #%lld", firstId]
                                                  style:UIAlertActionStyleDestructive
                                                handler:^(UIAlertAction *action) {
            [[CTDOCoreBridge shared] cancelScheduledForSlot:self.activeSlot schedId:firstId];
        }]];
    }
    [alert addAction:[UIAlertAction actionWithTitle:@"Đóng" style:UIAlertActionStyleCancel handler:nil]];
    [self presentViewController:alert animated:YES completion:nil];
}

- (void)onTapAutoReplyModal {
    NSArray<NSDictionary *> *rules = [[CTDOCoreBridge shared] listAutoReplyRules];
    NSMutableString *desc = [NSMutableString stringWithString:@"Danh sách quy tắc tự động trả lời (autoreply_rules.json #35):\n\n"];
    for (NSDictionary *r in rules) {
        BOOL en = [r[@"enabled"] boolValue];
        [desc appendFormat:@"%@ Từ khoá \"%@\" → \"%@\"\n\n",
         en ? @"✅ [BẬT]" : @"⏸ [TẮT]", r[@"pattern"], r[@"response"]];
    }
    UIAlertController *alert = [UIAlertController alertControllerWithTitle:@"🤖 Cấu hình Auto-Reply Bot (#35)"
                                                                   message:desc
                                                            preferredStyle:UIAlertControllerStyleAlert];
    [alert addTextFieldWithConfigurationHandler:^(UITextField *tf) {
        tf.placeholder = @"Từ khoá kích hoạt (ví dụ: xin chào)";
    }];
    [alert addTextFieldWithConfigurationHandler:^(UITextField *tf) {
        tf.placeholder = @"Nội dung tự động trả lời...";
    }];
    [alert addAction:[UIAlertAction actionWithTitle:@"➕ Thêm quy tắc mới" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        NSString *pat = alert.textFields[0].text ?: @"";
        NSString *resp = alert.textFields[1].text ?: @"";
        if (pat.length > 0 && resp.length > 0) {
            [[CTDOCoreBridge shared] mutateAutoReplyOp:1 patternOrId:pat response:resp matchMode:@"contains"];
        }
    }]];
    if (rules.count > 0) {
        NSString *firstId = rules.firstObject[@"id"] ?: @"rule_1";
        [alert addAction:[UIAlertAction actionWithTitle:@"🔄 Bật/Tắt quy tắc đầu tiên" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
            [[CTDOCoreBridge shared] mutateAutoReplyOp:2 patternOrId:firstId response:@"" matchMode:@"contains"];
        }]];
    }
    [alert addAction:[UIAlertAction actionWithTitle:@"Đóng" style:UIAlertActionStyleCancel handler:nil]];
    [self presentViewController:alert animated:YES completion:nil];
}

- (void)onTapThemeCycle {
    [[CTDOTheme shared] cycleTheme];
    [self applyTheme];
    [self refreshAllData];
}

- (void)onTapQRModal {
    NSDictionary *signedReq = [[CTDOCoreBridge shared] buildSignedRequestForService:@"friend"
                                                                             method:@"getListFriend"
                                                                         jsonParams:@"{\"page\":1,\"count\":50}"];
    NSArray *failover = signedReq[@"failover_hosts"] ?: @[];
    NSString *info = [NSString stringWithFormat:
                      @"• Phiên bản Core: v%@ (min iOS 12.0+)\n"
                      @"• Mã hoá: ParamCipher v2 (AES-128-CBC + AES-256-GCM #19, #16)\n"
                      @"• Signkey MD5: %@\n"
                      @"• Failover Hosts (#27): %@\n"
                      @"• Trạng thái QR: Sẵn sàng quét từ ứng dụng Zalo trên điện thoại",
                      [[CTDOCoreBridge shared] versionString],
                      signedReq[@"signkey"] ?: @"",
                      [failover componentsJoinedByString:@", "]];

    UIAlertController *alert = [UIAlertController alertControllerWithTitle:@"📱 Đăng nhập QR Zalo & Chẩn đoán Giao thức"
                                                                   message:info
                                                            preferredStyle:UIAlertControllerStyleAlert];
    [alert addAction:[UIAlertAction actionWithTitle:@"🔄 Làm mới hội thoại minh bạch (#23)" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        [self refreshAllData];
    }]];
    [alert addAction:[UIAlertAction actionWithTitle:@"Đóng" style:UIAlertActionStyleCancel handler:nil]];
    [self presentViewController:alert animated:YES completion:nil];
}

- (void)showHistoryMigrationModal {
    NSString *exported = [[CTDOCoreBridge shared] exportHistoryJSONForSlot:self.activeSlot];
    NSDictionary *importResult = [[CTDOCoreBridge shared] importHistoryJSON:exported forSlot:self.activeSlot];
    NSString *msg = [NSString stringWithFormat:
                     @"Đã kiểm tra đồng bộ & gộp lịch sử idempotent (#9):\n\n"
                     @"• Dung lượng JSON sao lưu: %lu bytes\n"
                     @"• Hội thoại mới thêm: %@\n"
                     @"• Tin nhắn mới thêm: %@\n"
                     @"• Bản ghi trùng lặp đã bỏ qua an toàn: %@",
                     (unsigned long)exported.length,
                     importResult[@"imported_threads"] ?: @0,
                     importResult[@"imported_messages"] ?: @0,
                     importResult[@"skipped_duplicates"] ?: @0];
    UIAlertController *alert = [UIAlertController alertControllerWithTitle:@"📦 Nhập / Xuất Lịch sử Tin nhắn (#9)"
                                                                   message:msg
                                                            preferredStyle:UIAlertControllerStyleAlert];
    [alert addAction:[UIAlertAction actionWithTitle:@"Sao chép JSON vào Clipboard" style:UIAlertActionStyleDefault handler:^(UIAlertAction *action) {
        [UIPasteboard generalPasteboard].string = exported;
    }]];
    [alert addAction:[UIAlertAction actionWithTitle:@"Đóng" style:UIAlertActionStyleCancel handler:nil]];
    [self presentViewController:alert animated:YES completion:nil];
}

// ============================================================================
// GitHub Releases API Auto-Update Notification & Detailed Changelog Modal
// ============================================================================
- (void)triggerGitHubUpdateCheck:(BOOL)simulateOldVersion {
    [[CTDOCoreBridge shared] checkGitHubLatestReleaseWithSimulateOldVersion:simulateOldVersion
                                                                 completion:^(NSDictionary *releaseInfo) {
        self.githubReleaseInfo = releaseInfo;
        BOOL hasUpdate = [releaseInfo[@"has_update"] boolValue];
        NSString *latestVer = releaseInfo[@"latest_version"] ?: @"0.0.5";
        NSString *curVer = releaseInfo[@"current_version"] ?: @"0.0.5";

        if (hasUpdate) {
            self.updateBannerDismissed = NO;
            self.updateBanner.hidden = NO;
            self.updateBannerLabel.text = [NSString stringWithFormat:@"🚀 Bản cập nhật mới ctdozlite v%@ trên GitHub API (hiện tại v%@)!", latestVer, curVer];
        } else {
            self.updateBanner.hidden = NO;
            self.updateBannerLabel.text = [NSString stringWithFormat:@"✅ Đang chạy bản mới nhất ctdozlite v%@ (đồng bộ từ GitHub Releases API)", latestVer];
        }
        [self.view setNeedsLayout];
    }];
}

- (void)onDismissUpdateBanner {
    self.updateBannerDismissed = YES;
    self.updateBanner.hidden = YES;
    [self.view setNeedsLayout];
}

- (void)onTapUpdateModal {
    if (!self.githubReleaseInfo) {
        [self triggerGitHubUpdateCheck:NO];
        return;
    }
    NSDictionary *info = self.githubReleaseInfo;
    BOOL hasUpdate = [info[@"has_update"] boolValue];
    NSString *latestVer = info[@"latest_version"] ?: @"0.0.5";
    NSString *curVer = info[@"current_version"] ?: @"0.0.5";
    NSString *title = info[@"title"] ?: @"ctdozlite v0.0.5";
    NSArray *bullets = [info[@"bullets"] isKindOfClass:[NSArray class]] ? info[@"bullets"] : @[];

    NSMutableString *bodyText = [NSMutableString stringWithFormat:
                                 @"Nguồn: api.github.com/repos/ctdoteam1110/ctdozlite/releases/latest\n"
                                 @"Trạng thái: %@\n"
                                 @"Tiêu đề: %@\n\n"
                                 @"📋 Chi tiết Sửa lỗi & Cập nhật:\n",
                                 hasUpdate
                                    ? [NSString stringWithFormat:@"🚀 Có bản mới v%@ (đang dùng v%@)", latestVer, curVer]
                                    : [NSString stringWithFormat:@"✅ Đã ở bản mới nhất v%@", latestVer],
                                 title];

    for (NSString *b in bullets) {
        [bodyText appendFormat:@"\n• %@", b];
    }

    UIAlertController *modal = [UIAlertController alertControllerWithTitle:[NSString stringWithFormat:@"🚀 GitHub Releases API — v%@", latestVer]
                                                                   message:bodyText
                                                            preferredStyle:UIAlertControllerStyleAlert];
    [modal addAction:[UIAlertAction actionWithTitle:@"🔔 Giả lập đang ở v0.0.4 để test báo Update v0.0.5"
                                              style:UIAlertActionStyleDefault
                                            handler:^(UIAlertAction *action) {
        [self triggerGitHubUpdateCheck:YES];
    }]];
    [modal addAction:[UIAlertAction actionWithTitle:@"🔄 Kiểm tra lại từ GitHub API (v0.0.5)"
                                              style:UIAlertActionStyleDefault
                                            handler:^(UIAlertAction *action) {
        [self triggerGitHubUpdateCheck:NO];
    }]];
    [modal addAction:[UIAlertAction actionWithTitle:@"🌐 Mở trang GitHub Release"
                                              style:UIAlertActionStyleDefault
                                            handler:^(UIAlertAction *action) {
        NSURL *url = [NSURL URLWithString:info[@"html_url"] ?: @"https://github.com/ctdoteam1110/ctdozlite/releases"];
        if (url) {
            [[UIApplication sharedApplication] openURL:url options:@{} completionHandler:nil];
        }
    }]];
    [modal addAction:[UIAlertAction actionWithTitle:@"Đóng" style:UIAlertActionStyleCancel handler:nil]];
    [self presentViewController:modal animated:YES completion:nil];
}

@end
