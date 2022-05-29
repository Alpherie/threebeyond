export default {
    en: {
        yes: 'yes',
        no: 'no',
        Disabled: 'Вisabled',
        disabled: 'disabled',
        enabled: 'enabled',
        Enabled: 'Enabled',
        minutes: 'minutes',

        timeLabel: {
            yesterday: '昨日',
            daysAgo: '日前',
            hoursAgo: '時前',
            minutesAgo: '分前',
            secondsAgo: '秒前',
        },

        rules: `
            <h1>デスカチャン</h1>
            <p>Welcome to the desuka.ch</p>
            <p>Desukachan is like a forum but without any identifiable marks. (it's called anonymous imageboard)</p>
            <p>You don't have to register or buy anything to access this site, all posts are created by (you) anonymous.</p>
            <p>Please follow rules below to save good environment for you and other users:</p>
            <p><span class="num">1</span>. <b>Don't post child porn or related content, admins of this site are also not responsible for your actions, please don't post it...</b></p>
            <p><span class="num">2</span>. <b>Don't spam, wipe, try to fill threads with shit and other unwanted stuff</b>.</p>
            <p><span class="num">3</span>. <b>Before creating thread, please try to find such threads (you can use the catalog mode). Duplicates of threads will be deleted.</b>.</p>
            <p><span class="num">4</span>. <b>Don't harm our service using DDOS-attacks, bot-spamming, etc.</b>.</p>
            <h3>Remember: we are not responsible for you actions and posts</h3>
            <h4>Some boards (may) have their own rules and opinions, please don't ignore them.</h4>
            <p>(There should be some text about the place where you can discuss this service, now it's in /ru/serv threads, but will be moved soon to /en/serv)</p>
            <h4>if you have complaint about post you can report it using actions-button on the right side of the post, if immediate actions should be aplied, please write a letter to <a href="mailto:madsuseki@protonmail.ch">madsuseki@protonmail.ch</a></h4>
            <h4>By clicking the button bellow you agree our rules and confirm you are over 18 years old.</h4>
        `,

        rulesAgree: 'LET ME IN [I agree the rules and my age is over 18 years]',

        writeform: {
            fileLoadFailed: {
                title: 'Failed to load file {name}',
                message: 'File has an invalid format'
            },
            attachments: 'Files',
            commentAndAttachments: 'Comments and attachments',
            location: 'Location',
            lang: 'Language',
            board: 'Board',
            thread: 'Thread',
            postInfo: 'Post information',
            name: 'Name',
            email: 'E-mail',
            subject: 'Subject',
            comment: 'Comment'
        },

        boardInfo: {
            'count': 'Threads',

            settings: {
                slowmode: "Slowmode",

                op: {
                    delete: 'OP can delete posts',
                    oppost: 'OP can create op-posts',
                },
                'tripcode_enabled': 'Tripcodes',
                'bumplimit': 'Bump limit',
                'last_replies': 'Last replies shown',
                'pages_count': 'Pages count',
                'per_page': 'Threads per page',
            }
        },

        commonButtons: {
            apply: 'Apply',
            close: 'Close',
            showPinnedPosts: "PINNED",
            replies: 'REPLIES',
            goThread: 'GO TO THREAD',
            cancel: 'Cancel',
            create: 'Create',
        },

        dcaptcha: {
            title: 'Captcha',
            description: 'Enter text from the image:',
            warning: 'This captcha type is automatically validated, if you entered captcha and it does\'not work, make sure you entered it valid.',
            requestAnother: 'Request another',
            generating: 'Captcha generation...',
            loadFailed: 'Failed to load captcha'
        },

        captchaKinds: {
            hcaptcha: 'HCaptcha',
            dcaptcha: 'Standart captcha'
        },

        banReasons: {
            'CP': 'Posting of requesting CP',
            'SPAM': 'Spamming, Flooding, Shitposting',
            'WIPE': 'Wiping',
            'OTHER': 'Other'
        },

        placeholders: {
            lang: 'Language',
            board: 'Board',
            postNum: 'Post num',
            minutes: 'Minutes',
            banReason: 'Reason',
            comment: 'Comment'
        },

        other: {
            fullscreen: {
                entered: {
                    title: "Fullscreen enabled",
                    text:  "To exit use `ESC` on desktops or two times back button."
                },

                leaved: {
                    title: "Fullscreen disabled",
                    text:  "To enable use fullscreen button."
                }
            }
        },

        context: {
            copyText: "Copy text",
            copyLink: "Copy link",

            post: {
                hide: "Hide post",
                show: "Show post" 
            },

            openRoute: {
                destroy: "Destroy"
            }
        },

        titles: {
            background: 'Background',
            safety: 'Security / Limits',
            captcha: 'Captcha',
            ui: "User Interface",
            fonts: "Fonts",
            colorSchemes: "Color schemes",
            boardList: "Board list",
            settings: "Settings",
            admin: "Administration"
        },

        boardList: {
            thereAreNoThreads: 'There are no threads',
            loadingThreads: "Loading threads"
        },

        settings: {
            previewSize: 'Attachment preview size',

            colorSchemes: {
                default: 'Default schemes',
                light: 'Light schemes',
                dark: 'Dark schemes',
                custom: 'Custom scheme',
                customPlaceholder: 'Hex of colors separated by comma',
                customMinLength: 'Scheme must have 8 or more colors',
                customInvalidFormat: 'Scheme has invalid format'
            },

            background: {
                enable: 'Enable background',
                url: 'Link to the background (URL)',
                blur: 'Blur',
                brightness: 'Brightness',
            },

            safety: {
                specialTagsLimit: {
                    title: "Special tags limit",
                    comment: "Maximum allowed special tags like urand/textwall/etc"
                },

                youtubePreloadLimit: {
                    title: "Max youtube links preload",
                    comment: "Maximum amount of youtube videos to preload"
                },

                imgurPreloadLimit: {
                    title: "Max imgur links preload",
                    comment: "Maximum amount of igmur pictures to preload"
                },

                repliesLimit: {
                    title: "Max replies in post",
                    comment: "Maximum amount of replies(>>1) in one post"
                }
            },

            admin: {
                guide: `Hello, thank you for helping our little service to grow!\nPlease don't break our rules and be responsible for your actions.\n\nYou can see available roles and permissions below.\nTypically they are named like langs::en::boards::b::moder / langs::en::boards::b::post_delete.\n\nIf seriously roles are not important for you. Check your permissions list to understand what you can do.\n\nTo understand what permissions label means you have to see on next pattern: 'langs::{code}::boards::{short}::{section}::{permission}'`,
                deauthSuccess: "You have successfuly deauthorized!",
                authorizationSuccess: "You have successfuly authorized!",
                authorizationFailed: "Failed to authorize",
                notAuthorized: "You are not authorized in the system.",
                roles: 'Roles',
                perms: 'Permissions',
            }
        },

        labels: {
            catalogMode: 'Catalog mode',
            favourite: 'Favourite',
            goBackToBoard: 'Go back',
            files: "files",
            clickOnLanguageToChoose: "Select language sector",
            dontSeeYourLanguage: "Don't see your language?",
            boardInformation: 'Board information',
            makeProposalLang: "Make a proposal",
            backToPages: 'Back to pages',
            refreshPages: 'Refresh',
            refreshCatalog: 'Catalog',
            nextPage: 'Load next page',
            createThread: 'Create thread',
            reportList: 'Reports',
            openThread: 'Open thread',
            hideThread: 'Hide thread',
            reportPending: 'PENDING',
            reportSolved: 'SOLVED',
            markSolved: 'Mark solved',
            reportsShowAll: 'Show all reports',
            reportsShowUnsolved: 'Show only pending reports'
        },

        dialogs: {
            openThread: {
                title: "Open thread",
                message: "Are you sure want to open the thread?"
            },

            closeThread: {
                title: "Close thread",
                message: "Are you sure want to close the thread?"
            },

            pinThread: {
                title: "Pin thread",
                message: "Are you sure want to pin the thread?"
            },

            unpinThread: {
                title: "Unpin thread",
                message: "Are you sure want to unpin the thread?"
            },

            makeThreadEndless: {
                title: "Make thread endless",
                message: "Are you sure want to make the thread endless?"
            },

            deleteMultiple: {
                title: "Delete posts",
                message: "Are you sure want to delete these posts?"
            },

            delete: {
                title: "Delete post",
                message: "Are you sure want to delete this post?"
            }
        },

        home: {
            header: "ですかちゃん",
            subheader: "A multilanguage mediaboard.",
            rules: "Rules",
            boards: "Boards",
            thanksLabel: 'Thanks everybody, who donated us any amount of money.',
            thanksGuide: 'You can donate BTC to <b>1CZVTK7rg8QKMJidvJFjzhX5CmSxTHDYBD</b><br />If you want to show your name in donations below, leave a comment.',
            thanksDonateBtn: 'Support us with money',
            sections: {
                general: {
                    name: "General",
                    content: {
                        total: {
                            domains: "Total languages",
                            boards: "Total boards",
                            mods: "Total mods",
                            donation: "Total donated"
                        },
                    }
                },

                posts: {
                    name: "Posts",
                    content: {
                        total: "Total posts",
                        last: {
                            hour: "Posts last hour",
                            day: "Posts last day",
                            week: "Posts last week",
                            month: "Posts last month",
                            year: "Posts last year",
                        },
                    }
                },

                media: {
                    name: "Media",
                    content: {
                        total: {
                            files: "Total files",
                            size: "Total size"
                        },
                    }
                },
            }
        },

        api: {
            unknown: "Unknown error",

            admin: {
                editingBoard: {
                    success: "The board successfuly edited",
                    failed: "Failed to edit the board"
                },

                creatingNewBoard: {
                    success: "Successfuly created new board",
                    failed: "Failed to create new board"
                },

                postPin: {
                    "success": "Post successfuly pinned",
                    "failed": "Failed to pin post"
                },
                postUnpin: {
                    "success": "Post successfuly unpinned",
                    "failed": "Failed to unpin post"
                },
                threadUnpin: {
                    "success": "Thread successfuly unpinned",
                    "failed": "Failed to open unpin thread"
                },

                threadPin: {
                    "success": "Thread successfuly pinned",
                    "failed": "Failed to open pin thread"
                },

                makeThreadEndless: {
                    "success": "Thread successfuly changed to endless type",
                    "failed": "Failed to change the thread type to endless"
                },

                threadOpen: {
                    "success": "Thread successfuly opened",
                    "failed": "Failed to open thread"
                },

                threadClose: {
                    "success": "Thread successfuly closed",
                    "failed": "Failed to close thread"
                },

                posterBan: {
                    success: 'Poster has just been successfuly banned.',
                    failed: 'Failed to ban poster'
                },

                postMultipleDelete: {
                    success: 'Posts have just been successfuly deleted',
                    failed: 'Failed to delete posts'
                },

                postDelete: {
                    success: 'Post has just been successfuly deleted',
                    failed: 'Failed to delete post'
                }
            },

            posting: {
                failed: "Failed to post",

                validate: {
                    noLang:    "No language selected",
                    noBoard:   "No board selected",
                    noComment: "Comment cannot be empty"
                },

                reply: {
                    success: "Successfuly created reply",
                    started: "Creating post...",
                    failed:  "Failed to reply"
                },

                thread: {
                    success: "Successfuly created thread",
                    started: "Creating thread...",
                    failed:  "Failed to create thread"
                }
            },

            posts: {
                getting: {
                    failed: 'Failed to get post'
                }
            },

            thread_creating: {
                slowmode: {
                    message: 'Board has slowmode enabled, you have to wait {left} minutes to create thread'
                }
            },

            threads: {
                reload: {
                    started: "Reloading thread...",
                    success: "Thread reloaded",
                    failed:  "Failed to reload thread",
                    newPosts: "New posts: {length}"
                },

                pageLoad: {
                    success: 'Page {page} loaded successfuly'
                },

                getStarted:  "Loading thread...",
                getFinished: "Thread loaded",
                getFailed:   "Failed to get thread"
            },

            boards: {
                loadPageStarted:  "Loading page...",
                loadPageFinished: "Page loaded",
                getPageFailed:    "Failed to get page",
                getAllFailed:     'Failed to get list of boards',
                getAllStarted:    'Loading board list...',
                getAllFinished:   'Board list loaded'
            },

            reporting: {
                solveFailed: 'Failed to solve report',
                getPageFailed: 'Failed to get reports',
                success: 'Report successfuly created',
                failed: 'Failed to create report'
            }
        },

        apiErrorReasons: {

        },

        apiErrorCodes: {
            'BOARD_ALREADY_EXISTS': 'Board already exists',
            'LAST_POST_TIMEOUT': 'You have to wait {secs} seconds before creating new post',
            'POST_NOT_FOUND': 'Post is not found',
            'CAPTCHA_VALUE_INVALID': 'Captcha value is invalid',
            'IP_BLACKLISTED': 'Your ip is blacklisted',
            'ACTION_NOT_ALLOWED_BY_BOARD_SETTINGS': 'Action is not allowed',
            'THREAD_SECRET_INVALID': 'Thread secret key is invalid',
            'MARKUP_ERROR': 'Markup error',
            'THREAD_CLOSED': 'Thread is closed',
            'HCAPTCHA_RESPONSE_REQUIRED': 'Captcha is required',
            'HCAPTCHA_RESPONSE_INVALID': 'Captcha is invalid',
            'ACCESS_MISSING_PERM': 'Operation not allowed',
            'MAX_REPORTS_SENT': 'You have reached max amount of reports. Wait 15 minutes and try again.',
            'VALIDATION_ERROR': 'Validation error',
            'undefined': "Unknown error",
            'AUTHORIZATION_BAD_TOKEN': 'Incorrect token',
            'BANNED': 'BAN<br />L | {lang}<br />B | {board}<br />U | {until}<br />R | {reason}<br />C | {comment}',
            'LANG_NOT_SUPPORTED': 'This language is not supported',
            'MULTIPART_UNKNOWN_EXTENSION': 'File format not supported',
            'BOARD_NOT_FOUND':  'Board does not exist',
            'PAGE_NOT_FOUND':   'Page does not exist',
            'THREAD_NOT_FOUND': 'Thread does not exist',
            'MULTIPART_FIELD_OVERFLOW': 'One of files has very big size.',
            'MULTIPART_FILES_LIMIT': 'Files amount limit reached'
        },

        blocker: {
            threadList: {
                reloading: "Reloading thread list..."
            },

            boards: {
                gettingAll: "Getting board list..."
            }
        },

        sidebar: {
            sections: {
                favouriteBoards: {
                    header: "Favourite boards",
                    button: "/{short}/ - {name}"
                },

                openPages: {
                    header: "Open pages",
                    button: "/{short}/{name}"
                },
            },
            buttons: {
                boardList: "Boards",
                history:   "History",
                settings:  "Settings"
            }
        },
        webClientInfo: {
            welcome: {
                title: "DESUKA.CH",
                text: "Welcome home."
            }
        },
    },
    
    ru: {
        yes: 'да',
        no: 'нет',
        Disabled: 'Выключено',
        disabled: 'выключено',
        enabled: 'включено',
        Enabled: 'Включено',
        minutes: 'минут',

        timeLabel: {
            yesterday: '昨日',
            daysAgo: '日前',
            hoursAgo: '時前',
            minutesAgo: '分前',
            secondsAgo: '秒前',
        },

        writeform: {
            fileLoadFailed: {
                title: 'Не удалось загрузить файл {name}',
                message: 'Файл имеет неправильный формат'
            },
            attachments: 'Файлы',
            commentAndAttachments: 'Комментарий и файлы',
            location: 'Положение',
            lang: 'Язык',
            board: 'Доска',
            thread: 'Тред',
            postInfo: 'Информация о посте',
            name: 'Имя',
            email: 'E-mail',
            subject: 'Тема',
            comment: 'Комментарий'
        },

        boardInfo: {
            'count': 'Всего тредов',

            settings: {
                slowmode: 'Слоумод',

                op: {
                    delete: 'ОП может удалять посты',
                    oppost: 'Оп может создавать оп-посты',
                },

                'tripcode_enabled': 'Трипкоды',
                'bumplimit': 'Бамплимит',
                'last_replies': 'Количество последних ответов на тред',
                'pages_count': 'Количество страниц',
                'per_page': 'Количество тредов на страницу',
            },
        },

        commonButtons: {
            apply: 'Применить',
            showPinnedPosts: "ЗАКРЕП",
            replies: 'ОТВЕТОВ',
            goThread: 'ПЕРЕЙТИ В ТРЕД',
            cancel: 'Отмена',
            close: 'Закрыть',
            create: 'Создать'
        },

        dcaptcha: {
            title: 'Капча',
            description: 'Введите текст с картинки:',
            warning: 'Эта капча проверяется в атоматическом режиме, если вы ввели капчу, и она не работает, убедитесь, что капча верна.',
            requestAnother: 'Запросить другую капчу',
            generating: 'Генерация капчи...',
            loadFailed: 'Не удалось загрузить капчу'
        },

        captchaKinds: {
            hcaptcha: 'HCaptcha',
            dcaptcha: 'Стандартная'
        },

        banReasons: {
            'CP': 'Постинг или реквест ЦП',
            'SPAM': 'Спам, Флуд, Щитпостинг',
            'WIPE': 'Вайп',
            'OTHER': 'Другое'
        },

        placeholders: {
            lang: 'Язык',
            board: 'Доска',
            postNum: 'Номер поста',
            minutes: 'Минуты',
            banReason: 'Причина',
            comment: 'Комментарий'
        },

        other: {
            fullscreen: {
                entered: {
                    title: "Включен полноэкранный режим",
                    text:  "To exit use `ESC` on desktops or two times back button."
                },

                leaved: {
                    title: "Полноэкранный режим отключен",
                    text:  "To enable use fullscreen button."
                }
            }
        },

        context: {
            copyText: "Скопировать текст",
            copyLink: "Скопировать ссылку",

            post: {
                hide: "Скрыть пост",
                show: "Показать пост"
            },

            openRoute: {
                destroy: "Уничтожить"
            }
        },

        titles: {
            background: 'Задний фон',
            safety: 'Безопасноть / Лимиты',
            captcha: 'Капча',
            ui: "Пользовательский интерфейс",
            fonts: "Шрифты",
            colorSchemes: "Цветовая схема",
            boardList: "Список досок",
            settings: "Настройки",
            admin: "Администрирование"
        },

        boardList: {
            thereAreNoThreads: 'Здесь нет тредов',
            loadingThreads: "Загружаем треды"
        },

        settings: {
            previewSize: 'Размер превью файла',
            background: {
                enable: 'Включить задний фон',
                url: 'Ссылка на изображение (URL)',
                blur: 'Размытие',
                brightness: 'Яркость',
            },

            colorSchemes: {
                default: 'Стандартные схемы',
                light: 'Светлые схемы',
                dark: 'Тёмные схемы',
                custom: 'Пользовательская схема',
                customPlaceholder: 'HEX-коды цветов через запятую',
                customMinLength: 'Схема должна иметь 8 или более цветов',
                customInvalidFormat: 'Формат схемы неправильный'
            },

            safety: {
                specialTagsLimit: {
                    title: "Максимум специальных тегов",
                    comment: "Максимальное количество разрешенных к обработке специальных тегов (textwall/urand/etc)"
                },

                youtubePreloadLimit: {
                    title: "Максимум youtube видео",
                    comment: "Максимальное количество презагрузок youtube видео"
                },

                imgurPreloadLimit: {
                    title: "Максимум imgur изображений",
                    comment: "Максимальное количество презагрузок imgur изображений"
                },

                repliesLimit: {
                    title: "Максимум ответов в одном посте",
                    comment: "Максимальное количество ответов (>>num) на одном посте"
                }
            },

            admin: {
                guide: `Здраствуйте, спасибо, что помогаете сервису!\nПожалуйста не злоупотребляйте правами админа.\n\nНиже показанны ваши права.\nОбычно они называются, как langs::en::boards::b::moder / langs::en::boards::b::post_delete.\n\n(тут должна быть куча предложения, но я не думаю что кому-то интересно смотреть на паттерн прав)'`,
                deauthSuccess: "Вы вышли",
                authorizationSuccess: "Вы авторизовались",
                authorizationFailed: "Не удалось авторизовать ключ",
                notAuthorized: "Вы не авторизованны",
                roles: 'Роли',
                perms: 'Права',
            }
        },

        labels: {
            catalogMode: 'Режим каталога',
            favourite: 'Избранное',
            goBackToBoard: 'Вернуться',
            files: "файлы",
            clickOnLanguageToChoose: "Нажмите на язык, чтоб перейти к его доскам:",
            dontSeeYourLanguage: "Хотите поддержку %языканейм%?",
            makeProposalLang: "Подать заявку",
            backToPages: 'Вернуться к страницам',
            refreshPages: 'Обновить',
            refreshCatalog: 'Каталог',
            nextPage: 'Загрузить следующую страницу',
            createThread: 'Создать тред',
            boardInformation: 'Информация о доске',
            reportList: 'Жалобы',
            openThread: 'Открыть тред',
            hideThread: 'Скрыть тред',
            reportPending: 'ОЖИДАЕТСЯ',
            reportSolved: 'РЕШЕННО',
            markSolved: 'Пометь как решенное',
            reportsShowAll: 'Показать все жалобы',
            reportsShowUnsolved: 'Показать только ожидаемые жалобы'
        },

        dialogs: {
            openThread: {
                title: "Открыть тред",
                message: "Вы действительно хотите открыть тред?"
            },

            closeThread: {
                title: "Закрыть тред",
                message: "Вы действительно хотите закрыть тред?"
            },

            pinThread: {
                title: "Закрепить тред",
                message: "Вы действительно хотите закрепить тред?"
            },

            unpinThread: {
                title: "Открепить тред",
                message: "Вы действительно хотите удалить открепить тред?"
            },

            makeThreadEndless: {
                title: "Сделать тред бесконечным",
                message: "Вы действительно хотите сделать тред бесконечным?"
            },

            deleteMultiple: {
                title: "Удаление постов",
                message: "Вы действительно хотите удалить выбранные посты?"
            },

            delete: {
                title: "Удалить пост",
                message: "Вы действительно хотите удалить выбранный пост?"
            }
        },

        home: {
            header: "ですかちゃん",
            subheader: "Еще одна имиджборда",
            rules: "Правила",
            boards: "Доски",
            thanksLabel: 'Спасибо всем!',
            thanksDonateBtn: 'Поддержать проект',
            thanksGuide: 'Вы можете положить биткоин на кошелек <b>1CZVTK7rg8QKMJidvJFjzhX5CmSxTHDYBD</b><br />Если хотите, чтобы ваше имя было показанно ниже в донатах, оставьте комментарий к платежу.',
            sections: {
                general: {
                    name: "Общее",
                    content: {
                        total: {
                            domains: "Всего языков",
                            boards: "Всего досок",
                            mods: "Всего модераторов",
                            donation: "Всего пожертвованно"
                        },
                    }
                },

                posts: {
                    name: "Посты",
                    content: {
                        total: "Всего постов",
                        last: {
                            hour: "Постов за час",
                            day: "Постов за день",
                            week: "Постов за неделю",
                            month: "Постов за месяц",
                            year: "Постов за год",
                        },
                    }
                },

                media: {
                    name: "Медиа",
                    content: {
                        total: {
                            files: "Всего файлов",
                            size: "Общий размер"
                        },
                    }
                },
            }
        },

        api: {
            unknown: "Неизвестаня ошибка",

            thread_creating: {
                slowmode: {
                    message: 'На доске включен слоумод, вам нужно подождать {left} минут, чтобы создать тред'
                }
            },

            admin: {
                editingBoard: {
                    success: "Доска успешно изменена",
                    failed: "Не удалось изменить доску"
                },

                creatingNewBoard: {
                    success: "Доска успешно создана",
                    failed: "Не удалось создать доску"
                },

                postPin: {
                    "success": "Пост успешно закреплен",
                    "failed": "Не удалось закрепить пост"
                },
                postUnpin: {
                    "success": "Пост успешно откреплен",
                    "failed": "Не удалось открепить пост"
                },
                threadUnpin: {
                    "success": "Тред успешно откреплен",
                    "failed": "Не удалось открепить тред"
                },

                threadPin: {
                    "success": "Тред успешно закреплен",
                    "failed": "Не удалось закрепить тред"
                },

                makeThreadEndless: {
                    "success": "Тред успешно изменен в бесконечный",
                    "failed": "Не удалось сделать тред бесконечным"
                },

                threadOpen: {
                    "success": "Тред успешно открыт",
                    "failed": "Не удалось открыть тред"
                },

                threadClose: {
                    "success": "Тред успешно закрыт",
                    "failed": "Не удалось закрыть тред"
                },

                posterBan: {
                    success: 'Постер был успешно забанен',
                    failed: 'Не удалось забанить постера'
                },

                postMultipleDelete: {
                    success: 'Посты были успешно удаленны',
                    failed: 'Не удалось удалить посты'
                },

                postDelete: {
                    success: 'Пост был успешно удален',
                    failed: 'Не удалось удалить пост'
                }
            },

            posting: {
                failed: "Не удалось создать пост",

                validate: {
                    noLang:    "Язык не выбран",
                    noBoard:   "Доска не выбрана",
                    noComment: "Комментарий пустой"
                },

                reply: {
                    success: "Ответ успешно создан",
                    started: "Отвечаем...",
                    failed:  "Не удалось создать ответ"
                },

                thread: {
                    success: "Тред успешно создан",
                    started: "Создаем тред...",
                    failed:  "Не удалось создать тред"
                }
            },

            posts: {
                getting: {
                    failed: 'Не удалось загрузить пост'
                }
            },

            threads: {
                reload: {
                    started: "Перезагружаем тред",
                    success: "Тред перезагружен",
                    failed:  "Не удалось перезагрузить тред",
                    newPosts: "Новых постов: {length}"
                },

                pageLoad: {
                    success: 'Страница {page} успешно загружена'
                },

                getStarted:  "Загружаем тред",
                getFinished: "Тред загружен",
                getFailed:   "Не удалось загрузить тред"
            },

            boards: {
                loadPageStarted:  "Загрузка страницы",
                loadPageFinished: "Страница загружена",
                getPageFailed:    "Не удалось загрузить страницу",
                getAllFailed:     'Не удалось получить список досок',
                getAllStarted:    'Загрузка списка досок',
                getAllFinished:   'Список досок загружен'
            },

            reporting: {
                solveFailed: 'Не удалось решить жалобу',
                getPageFailed: 'Не удалось получить жалобы',
                success: 'Жалоба успешно создана',
                failed: 'Не удалось создать жалобу'
            }
        },

        apiErrorReasons: {

        },

        apiErrorCodes: {
            'BOARD_ALREADY_EXISTS': 'Доска уже существует',
            'LAST_POST_TIMEOUT': 'Вам нужно подождать {secs} секунд перед созданием нового поста.',
            'POST_NOT_FOUND': 'Пост не найден',
            'CAPTCHA_VALUE_INVALID': 'Значение капчи неправильное',
            'IP_BLACKLISTED': 'Ваш IP находится в черном списке',
            'ACTION_NOT_ALLOWED_BY_BOARD_SETTINGS': 'Действие запрещенно',
            'THREAD_SECRET_INVALID': 'Неверный секретный ключ треда',
            'MARKUP_ERROR': 'Ошибка разметки',
            'THREAD_CLOSED': 'Тред закрыт',
            'HCAPTCHA_RESPONSE_REQUIRED': 'Решите капчу',
            'HCAPTCHA_RESPONSE_INVALID': 'Капча неверная',
            'ACCESS_MISSING_PERM': 'Отказано в доступе',
            'MAX_REPORTS_SENT': 'Вы уже жаловались максимальное количество раз. Попробуйте через 15 минут.',
            'VALIDATION_ERROR': 'Ошибка валидации',
            'undefined': "Неизвестаня ошибка",
            'AUTHORIZATION_BAD_TOKEN': 'Не правильный ключ',
            'BANNED': 'БАН<br />L | {lang}<br />B | {board}<br />U | {until}<br />R | {reason}<br />C | {comment}',
            'LANG_NOT_SUPPORTED': 'Язык не поддерживается',
            'MULTIPART_UNKNOWN_EXTENSION': 'Формат файла не поддерживается',
            'BOARD_NOT_FOUND':  'Доска не существует',
            'PAGE_NOT_FOUND':   'Страницу не существует',
            'THREAD_NOT_FOUND': 'Тред не существует',
            'MULTIPART_FIELD_OVERFLOW': 'Один из файлов превышает лимит.',
            'MULTIPART_FILES_LIMIT': 'Количество файлов превышает допустимый лимит',
            'MULTIPART_FILE_CORRUPTED': 'Файл поврежден',
        },

        blocker: {
            threadList: {
                reloading: "Загрузка списка тредов"
            },

            boards: {
                gettingAll: "Загрузка списка досок"
            }
        },

        sidebar: {
            sections: {
                favouriteBoards: {
                    header: "Избранные доски",
                    button: "/{short}/ - {name}"
                },

                openPages: {
                    header: "Открытые страницы",
                    button: "/{short}/{name}"
                },
            },
            buttons: {
                boardList: "Доски",
                history:   "История",
                settings:  "Настройки"
            }
        },
        webClientInfo: {
            welcome: {
                title: "DESUKA.CH",
                text: "Добро пожаловать."
            }
        },

        rules: `
            <h1>デスカチャン</h1>
            <p>Добро пожаловать на desuka.ch</p>
            <p>Desukachan это система досок, где любое мнение имеет право на жизнь.</p>
            <p>Сайт не требует регистрации, все публикации на сайте являются анонимными.</p>
            <p>Но для сохранения работоспособности сервиса, необходимо соблюдать определенные правила:</p>
            <p>1. <b>Запрещено публиковать детскую порнографию, детскую эротику, <br />имитацию детской порнографии, любые материалы, которые могут привести к блокировке сайта</b>. <br />- За нарушения правила может быть выдан перманентный бан по подсети. (а еще донос напишем)</p>
            <p>2. <b>Запрещено вайпать треды - пытаться уничтожить тред путем его засорения бесполезными ответами</b>.</p>
            <p>3. <b>Запрещено спамить/флудить/создавать дубликаты еще живущих тредов</b>.</p>
            <p>4. <b>Запрещено ддосить, причинять вред или призывать к нарушению правил сервиса</b>.</p>
            <h3>Помните, администрация не несет ответственность за оставленные пользователями публикации!</h3>
            <h4>Каждый раздел может иметь свои внутренние правила, например бан аваторок, пожалуйста не игнорируйте их, если таковы имеются.</h4>
            <p>Обсуждение работы сервиса происходит в специальных тредах /ru/serv</p>
            <h4>Если у вас возникла жалоба, вы можете пожаловаться через кнопку справа от поста. Так же можете написать на почту: <a href="mailto:madsuseki@protonmail.ch">madsuseki@protonmail.ch</a></h4>
            <h4>Сайт предназначен для лиц старше 18 лет.</h4>
        `,

        rulesTime: 'Правила прочитай, а не тыкай на зеленые кнопочки!',
        rulesAgree: 'Я прочитал и согласен с правилами сервиса',
        rulesOk: 'Принять правила'
    }
}
