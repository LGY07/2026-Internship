#include "ApiClient.h"

#include <QJsonDocument>
#include <QNetworkReply>
#include <QNetworkRequest>
#include <QUrl>

// ── Construction ──────────────────────────────────────────────────────

ApiClient::ApiClient(QObject *parent)
    : QObject(parent)
    , m_manager(new QNetworkAccessManager(this))
{
}

QString ApiClient::baseUrl() const       { return m_baseUrl; }
QString ApiClient::accessToken() const   { return m_accessToken; }
QString ApiClient::permissions() const   { return m_permissions; }

void ApiClient::setBaseUrl(const QString &url)
{
    if (m_baseUrl != url) {
        m_baseUrl = url;
        emit baseUrlChanged();
    }
}

// ── Private helpers ───────────────────────────────────────────────────

QNetworkRequest ApiClient::createRequest(const QUrl &url) const
{
    QNetworkRequest req(url);
    req.setHeader(QNetworkRequest::ContentTypeHeader, QStringLiteral("application/json"));
    if (!m_accessToken.isEmpty())
        req.setRawHeader("Authorization", ("Bearer " + m_accessToken).toUtf8());
    return req;
}

void ApiClient::handleReply(QNetworkReply *reply, JsonCallback onSuccess, ErrorCallback onFailure)
{
    reply->deleteLater();

    const QByteArray data = reply->readAll();
    const QJsonObject obj = QJsonDocument::fromJson(data).object();

    // Always prefer the server's JSON "message" field
    const QString serverMsg = obj.value(QStringLiteral("message")).toString();

    if (reply->error() != QNetworkReply::NoError) {
        if (onFailure) onFailure(serverMsg.isEmpty() ? reply->errorString() : serverMsg);
        return;
    }

    if (!obj.value(QStringLiteral("success")).toBool(false)) {
        if (onFailure) onFailure(serverMsg);
        return;
    }

    if (onSuccess) onSuccess(obj);
}

void ApiClient::httpGet(const QString &path, JsonCallback onSuccess, ErrorCallback onFailure)
{
    QNetworkReply *r = m_manager->get(createRequest(QUrl(m_baseUrl + path)));
    connect(r, &QNetworkReply::finished, this, [=]() { handleReply(r, onSuccess, onFailure); });
}

void ApiClient::httpPost(const QString &path, const QJsonObject &body, JsonCallback onSuccess, ErrorCallback onFailure)
{
    QNetworkReply *r = m_manager->post(createRequest(QUrl(m_baseUrl + path)),
                                       QJsonDocument(body).toJson(QJsonDocument::Compact));
    connect(r, &QNetworkReply::finished, this, [=]() { handleReply(r, onSuccess, onFailure); });
}

void ApiClient::httpPut(const QString &path, const QJsonObject &body, JsonCallback onSuccess, ErrorCallback onFailure)
{
    QNetworkReply *r = m_manager->put(createRequest(QUrl(m_baseUrl + path)),
                                      QJsonDocument(body).toJson(QJsonDocument::Compact));
    connect(r, &QNetworkReply::finished, this, [=]() { handleReply(r, onSuccess, onFailure); });
}

void ApiClient::httpPatch(const QString &path, const QJsonObject &body, JsonCallback onSuccess, ErrorCallback onFailure)
{
    QNetworkReply *r = m_manager->sendCustomRequest(createRequest(QUrl(m_baseUrl + path)),
                                                    "PATCH",
                                                    QJsonDocument(body).toJson(QJsonDocument::Compact));
    connect(r, &QNetworkReply::finished, this, [=]() { handleReply(r, onSuccess, onFailure); });
}

void ApiClient::httpDelete(const QString &path, JsonCallback onSuccess, ErrorCallback onFailure)
{
    QNetworkReply *r = m_manager->deleteResource(createRequest(QUrl(m_baseUrl + path)));
    connect(r, &QNetworkReply::finished, this, [=]() { handleReply(r, onSuccess, onFailure); });
}

// ── Authentication ────────────────────────────────────────────────────

void ApiClient::login(const QString &username, const QString &password)
{
    QJsonObject body;
    body[QStringLiteral("username")] = username;
    body[QStringLiteral("password")] = password;

    httpPost(QStringLiteral("/api/login"), body,
        [this](const QJsonObject &obj) {
            m_accessToken = obj.value(QStringLiteral("access_token")).toString();
            m_permissions = obj.value(QStringLiteral("permissions")).toString();
            emit accessTokenChanged();
            emit permissionsChanged();
            emit loginSucceeded(m_accessToken, m_permissions);
        },
        [this](const QString &msg) { emit loginFailed(msg); });
}

void ApiClient::registerUser(const QString &username, const QString &password)
{
    QJsonObject body;
    body[QStringLiteral("username")] = username;
    body[QStringLiteral("password")] = password;

    httpPost(QStringLiteral("/api/register"), body,
        [this](const QJsonObject &) { emit registerSucceeded(); },
        [this](const QString &msg) { emit registerFailed(msg); });
}

void ApiClient::logout()
{
    m_accessToken.clear();
    m_permissions.clear();
    emit accessTokenChanged();
    emit permissionsChanged();
}

// ── Books ─────────────────────────────────────────────────────────────

void ApiClient::fetchBooks(int page)
{
    httpGet(QStringLiteral("/api/books?page=%1").arg(page),
        [this](const QJsonObject &obj) {
            emit booksFetched(obj.value(QStringLiteral("books")).toArray());
        },
        [this](const QString &msg) { emit booksFetchFailed(msg); });
}

void ApiClient::searchBooks(const QString &title, const QString &author, const QString &category)
{
    QString path = QStringLiteral("/api/books/search?");
    if (!title.isEmpty())    path += QStringLiteral("title=%1&").arg(title);
    if (!author.isEmpty())   path += QStringLiteral("author=%1&").arg(author);
    if (!category.isEmpty()) path += QStringLiteral("category=%1&").arg(category);
    path.chop(1); // remove trailing '&' or '?'

    httpGet(path,
        [this](const QJsonObject &obj) {
            emit booksFetched(obj.value(QStringLiteral("books")).toArray());
        },
        [this](const QString &msg) { emit booksFetchFailed(msg); });
}

void ApiClient::addBook(const QString &title, const QString &author,
                        const QJsonArray &categories, const QString &description,
                        const QString &publishedDate)
{
    QJsonObject body;
    body[QStringLiteral("title")]        = title;
    body[QStringLiteral("author")]       = author;
    body[QStringLiteral("category")]     = categories;
    if (!description.isEmpty())    body[QStringLiteral("description")]    = description;
    if (!publishedDate.isEmpty())  body[QStringLiteral("published_date")] = publishedDate;

    httpPost(QStringLiteral("/api/books"), body,
        [this](const QJsonObject &) { emit bookAdded(); },
        [this](const QString &msg) { emit bookAddFailed(msg); });
}

void ApiClient::removeBook(int id)
{
    httpDelete(QStringLiteral("/api/books/%1").arg(id),
        [this](const QJsonObject &) { emit bookRemoved(); },
        [this](const QString &msg) { emit bookRemoveFailed(msg); });
}

// ── Users ─────────────────────────────────────────────────────────────

void ApiClient::fetchCurrentUser()
{
    httpGet(QStringLiteral("/api/users/me"),
        [this](const QJsonObject &obj) {
            emit currentUserFetched(obj.value(QStringLiteral("user")).toObject());
        },
        [this](const QString &msg) { emit currentUserFetchFailed(msg); });
}

void ApiClient::fetchAllUsers()
{
    httpGet(QStringLiteral("/api/users"),
        [this](const QJsonObject &obj) {
            emit allUsersFetched(obj.value(QStringLiteral("users")).toArray());
        },
        [this](const QString &msg) { emit allUsersFetchFailed(msg); });
}

void ApiClient::searchUsers(const QString &keyword)
{
    httpGet(QStringLiteral("/api/users/%1").arg(keyword),
        [this](const QJsonObject &obj) {
            emit usersFound(obj.value(QStringLiteral("users")).toArray());
        },
        [this](const QString &msg) { emit usersSearchFailed(msg); });
}

void ApiClient::removeUser(int id)
{
    httpDelete(QStringLiteral("/api/users/%1").arg(id),
        [this](const QJsonObject &) { emit userRemoved(); },
        [this](const QString &msg) { emit userRemoveFailed(msg); });
}

void ApiClient::changePassword(const QString &userId, const QString &newPassword)
{
    QJsonObject body;
    body[QStringLiteral("password")] = newPassword;

    httpPut(QStringLiteral("/api/users/%1/password").arg(userId), body,
        [this](const QJsonObject &) { emit passwordChanged(); },
        [this](const QString &msg) { emit passwordChangeFailed(msg); });
}

void ApiClient::updateUserInfo(int id, const QString &phone, const QString &email)
{
    QJsonObject body;
    body[QStringLiteral("phone")] = phone;
    body[QStringLiteral("email")] = email;

    httpPatch(QStringLiteral("/api/users/%1").arg(id), body,
        [this](const QJsonObject &) { emit userInfoUpdated(); },
        [this](const QString &msg) { emit userInfoUpdateFailed(msg); });
}

// ── Borrow ────────────────────────────────────────────────────────────

void ApiClient::borrowBook(int bookId, int userId, const QString &expireDate)
{
    QJsonObject body;
    body[QStringLiteral("user_id")]     = userId;
    body[QStringLiteral("expire_date")] = expireDate;

    httpPost(QStringLiteral("/api/books/%1/borrow").arg(bookId), body,
        [this](const QJsonObject &) { emit bookBorrowed(); },
        [this](const QString &msg) { emit bookBorrowFailed(msg); });
}

void ApiClient::returnBook(int bookId, int userId)
{
    QJsonObject body;
    body[QStringLiteral("user_id")] = userId;

    httpPost(QStringLiteral("/api/books/%1/return").arg(bookId), body,
        [this](const QJsonObject &) { emit bookReturned(); },
        [this](const QString &msg) { emit bookReturnFailed(msg); });
}

void ApiClient::fetchBookBorrowRecords(int bookId)
{
    httpGet(QStringLiteral("/api/books/%1/borrow").arg(bookId),
        [this](const QJsonObject &obj) {
            emit borrowRecordsFetched(obj.value(QStringLiteral("borrow_records")).toArray());
        },
        [this](const QString &msg) { emit borrowRecordsFetchFailed(msg); });
}

void ApiClient::fetchMyBorrowRecords()
{
    httpGet(QStringLiteral("/api/users/me/borrow"),
        [this](const QJsonObject &obj) {
            emit borrowRecordsFetched(obj.value(QStringLiteral("borrow_records")).toArray());
        },
        [this](const QString &msg) { emit borrowRecordsFetchFailed(msg); });
}

void ApiClient::fetchUserBorrowRecords(int userId)
{
    httpGet(QStringLiteral("/api/users/%1/borrow").arg(userId),
        [this](const QJsonObject &obj) {
            emit borrowRecordsFetched(obj.value(QStringLiteral("borrow_records")).toArray());
        },
        [this](const QString &msg) { emit borrowRecordsFetchFailed(msg); });
}

void ApiClient::fetchExpiredBorrowRecords(const QString &expiredAfter)
{
    QString path = QStringLiteral("/api/books/borrow/expired");
    if (!expiredAfter.isEmpty())
        path += QStringLiteral("?expired_after=%1").arg(expiredAfter);

    httpGet(path,
        [this](const QJsonObject &obj) {
            emit borrowRecordsFetched(obj.value(QStringLiteral("borrow_records")).toArray());
        },
        [this](const QString &msg) { emit borrowRecordsFetchFailed(msg); });
}

void ApiClient::fetchAllBorrowRecords()
{
    httpGet(QStringLiteral("/api/books/borrow"),
        [this](const QJsonObject &obj) {
            emit borrowRecordsFetched(obj.value(QStringLiteral("borrow_records")).toArray());
        },
        [this](const QString &msg) { emit borrowRecordsFetchFailed(msg); });
}

// ── Renewals ──────────────────────────────────────────────────────────

void ApiClient::submitRenewal(int borrowRecordId, const QString &expiredAfter)
{
    QJsonObject body;
    body[QStringLiteral("expired_after")] = expiredAfter;

    httpPost(QStringLiteral("/api/books/borrow/%1/renew").arg(borrowRecordId), body,
        [this](const QJsonObject &) { emit renewalSubmitted(); },
        [this](const QString &msg) { emit renewalSubmitFailed(msg); });
}

void ApiClient::fetchRenewals()
{
    httpGet(QStringLiteral("/api/books/borrow/renewals"),
        [this](const QJsonObject &obj) {
            emit renewalsFetched(obj.value(QStringLiteral("renewals")).toArray());
        },
        [this](const QString &msg) { emit renewalsFetchFailed(msg); });
}

void ApiClient::approveRenewal(int renewalId, bool approved)
{
    QJsonObject body;
    body[QStringLiteral("approved")] = approved;

    httpPost(QStringLiteral("/api/books/borrow/renewals/%1/approve").arg(renewalId), body,
        [this](const QJsonObject &) { emit renewalApproved(); },
        [this](const QString &msg) { emit renewalApproveFailed(msg); });
}
