#ifndef APICLIENT_H
#define APICLIENT_H

#include <QObject>
#include <QNetworkAccessManager>
#include <functional>
#include <QJsonArray>
#include <QJsonObject>

class QNetworkReply;

class ApiClient : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QString baseUrl   READ baseUrl   WRITE setBaseUrl   NOTIFY baseUrlChanged)
    Q_PROPERTY(QString accessToken READ accessToken                NOTIFY accessTokenChanged)
    Q_PROPERTY(QString permissions READ permissions                NOTIFY permissionsChanged)

public:
    explicit ApiClient(QObject *parent = nullptr);

    QString baseUrl() const;
    void    setBaseUrl(const QString &url);

    QString accessToken() const;
    QString permissions() const;
    bool    isLoggedIn() const { return !m_accessToken.isEmpty(); }
    bool    isAdmin()    const { return m_permissions == QStringLiteral("manager"); }

    // ── Authentication ───────────────────────────────────────────────
    Q_INVOKABLE void login(const QString &username, const QString &password);
    Q_INVOKABLE void registerUser(const QString &username, const QString &password);
    Q_INVOKABLE void logout();

    // ── Books ────────────────────────────────────────────────────────
    Q_INVOKABLE void fetchBooks(int page = 1);
    Q_INVOKABLE void searchBooks(const QString &title   = QString(),
                                 const QString &author  = QString(),
                                 const QString &category = QString());
    Q_INVOKABLE void addBook(const QString &title,
                             const QString &author,
                             const QJsonArray &categories,
                             const QString &description    = QString(),
                             const QString &publishedDate  = QString());
    Q_INVOKABLE void removeBook(int id);

    // ── Users ────────────────────────────────────────────────────────
    Q_INVOKABLE void fetchCurrentUser();
    Q_INVOKABLE void fetchAllUsers();
    Q_INVOKABLE void searchUsers(const QString &keyword);
    Q_INVOKABLE void removeUser(int id);
    Q_INVOKABLE void changePassword(const QString &userId, const QString &newPassword);
    Q_INVOKABLE void updateUserInfo(int id, const QString &phone, const QString &email);

    // ── Borrow ───────────────────────────────────────────────────────
    Q_INVOKABLE void borrowBook(int bookId, int userId, const QString &expireDate);
    Q_INVOKABLE void returnBook(int bookId, int userId);
    Q_INVOKABLE void fetchBookBorrowRecords(int bookId);
    Q_INVOKABLE void fetchMyBorrowRecords();
    Q_INVOKABLE void fetchUserBorrowRecords(int userId);
    Q_INVOKABLE void fetchExpiredBorrowRecords(const QString &expiredAfter = QString());
    Q_INVOKABLE void fetchAllBorrowRecords();

    // ── Renewals ─────────────────────────────────────────────────────
    Q_INVOKABLE void submitRenewal(int borrowRecordId, const QString &expiredAfter);
    Q_INVOKABLE void fetchRenewals();
    Q_INVOKABLE void approveRenewal(int renewalId, bool approved);

signals:
    void baseUrlChanged();
    void accessTokenChanged();
    void permissionsChanged();

    // Auth
    void loginSucceeded(const QString &token, const QString &permissions);
    void loginFailed(const QString &message);
    void registerSucceeded();
    void registerFailed(const QString &message);

    // Books
    void booksFetched(const QJsonArray &books);
    void booksFetchFailed(const QString &message);
    void bookAdded();
    void bookAddFailed(const QString &message);
    void bookRemoved();
    void bookRemoveFailed(const QString &message);

    // Users
    void currentUserFetched(const QJsonObject &user);
    void currentUserFetchFailed(const QString &message);
    void allUsersFetched(const QJsonArray &users);
    void allUsersFetchFailed(const QString &message);
    void usersFound(const QJsonArray &users);
    void usersSearchFailed(const QString &message);
    void userRemoved();
    void userRemoveFailed(const QString &message);
    void passwordChanged();
    void passwordChangeFailed(const QString &message);
    void userInfoUpdated();
    void userInfoUpdateFailed(const QString &message);

    // Borrow
    void bookBorrowed();
    void bookBorrowFailed(const QString &message);
    void bookReturned();
    void bookReturnFailed(const QString &message);
    void borrowRecordsFetched(const QJsonArray &records);
    void borrowRecordsFetchFailed(const QString &message);

    // Renewals
    void renewalSubmitted();
    void renewalSubmitFailed(const QString &message);
    void renewalsFetched(const QJsonArray &renewals);
    void renewalsFetchFailed(const QString &message);
    void renewalApproved();
    void renewalApproveFailed(const QString &message);

private:
    using JsonCallback  = std::function<void(const QJsonObject &)>;
    using ErrorCallback = std::function<void(const QString &)>;

    QNetworkRequest createRequest(const QUrl &url) const;
    void handleReply(QNetworkReply *reply, JsonCallback onSuccess, ErrorCallback onFailure);

    void httpGet   (const QString &path, JsonCallback, ErrorCallback);
    void httpPost  (const QString &path, const QJsonObject &body, JsonCallback, ErrorCallback);
    void httpPut   (const QString &path, const QJsonObject &body, JsonCallback, ErrorCallback);
    void httpPatch (const QString &path, const QJsonObject &body, JsonCallback, ErrorCallback);
    void httpDelete(const QString &path, JsonCallback, ErrorCallback);

    QNetworkAccessManager *m_manager;
    QString m_baseUrl;
    QString m_accessToken;
    QString m_permissions;
};

#endif // APICLIENT_H
