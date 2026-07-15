cmake_minimum_required(VERSION 3.16)

project(LibraryManager VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 REQUIRED COMPONENTS Quick QuickControls2 Network LinguistTools)

qt_standard_project_setup(REQUIRES 6.10)

# Content of downloaded design from Figma to Qt or similar
if (EXISTS "${CMAKE_CURRENT_LIST_DIR}/importedcontent/CMakeLists.txt")
    add_subdirectory(${CMAKE_CURRENT_LIST_DIR}/importedcontent)
endif()

qt_add_executable(appLibraryManager
    main.cpp
    ApiClient.h
    ApiClient.cpp
)

qt_add_qml_module(appLibraryManager
    URI LibraryManager
    QML_FILES
        Main.qml
        LoginPage.qml
        UserPage.qml
        AdminPage.qml
        pages/user/UserBooksPage.qml
        pages/user/UserBorrowsPage.qml
        pages/user/UserRenewalsPage.qml
        pages/user/UserProfilePage.qml
        pages/admin/AdminBooksPage.qml
        pages/admin/AdminBorrowPage.qml
        pages/admin/AdminReturnPage.qml
        pages/admin/AdminRecordsPage.qml
        pages/admin/AdminRenewalsPage.qml
        pages/admin/AdminUsersPage.qml
        pages/admin/AdminProfilePage.qml
        Toast.qml
    RESOURCES
)

# ── Internationalization ────────────────────────────────────────────
qt_add_translations(appLibraryManager
    TS_FILES
        i18n/LibraryManager_en.ts
        i18n/LibraryManager_zh.ts
    RESOURCE_PREFIX "/i18n"
)

target_link_libraries(appLibraryManager
    PRIVATE Qt6::Quick
    PRIVATE Qt6::QuickControls2
    PRIVATE Qt6::Network
)

install(TARGETS appLibraryManager
    BUNDLE DESTINATION .
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)
