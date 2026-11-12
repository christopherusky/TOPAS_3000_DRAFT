# How to Give Feedback on TOPAS 3000A

This page is for anyone who wants to comment on the TOPAS 3000A specification, the buyer's guide or any other file in this repository, and who has not used GitHub before. You do not need to know how to code. You do not need to install anything.

Every way of commenting below happens in your web browser.

> **Repository address:** `https://github.com/christopherusky/TOPAS_3000_DRAFT`
> Wherever this page says "the repository", it means that address.

---

## 1. Before you start: what you need

| You need | Notes |
|---|---|
| A web browser | Any current browser works. |
| A free GitHub account | Needed to *write* a comment. You can *read* everything without one. |

### Creating a GitHub account (about two minutes)

1. Go to <https://github.com/signup>.
2. Enter an email address, a password and a username. Your username is public. You can use a work or personal address, and you can choose a username that does not include your name.
3. Complete the puzzle and confirm your email address using the message GitHub sends you.
4. You can skip any questions about your interests or plans.

Anything you write in a comment is **public**, along with your username. Do not include passwords, personal data, or confidential information about your authority's systems or suppliers.

---

## 2. Which method should I use?

| I want to... | Use | Section |
|---|---|---|
| Ask a question, report a mistake or make a general comment | **Open an issue** | 3 |
| Comment on one particular sentence, table row or line in a file | **Comment on a line** | 4 |
| Reply to something someone else has already said | **Reply to an issue** | 5 |
| Suggest the exact new wording | **Open an issue with your wording** | 6 |

If you are unsure, use an **issue** (section 3). It always works.

---

## 3. Open an issue (the simplest way)

An issue is a titled message that the maintainers, and everyone else, can see and reply to.

1. Go to the repository.
2. Click the **Issues** tab near the top of the page.
3. Before you write a new one, type a few words into the search box. Someone may already have raised the same point. If so, open it and add your view as a reply (see section 5).
4. Click the green **New issue** button.
5. Fill in:
   - **Title** — a short summary, for example *"Buyer's guide 3.2: unclear what "native publishing" means for a UTC system"*.
   - **Description** — see "What makes feedback easy to act on" below.
6. Click **Submit new issue**.

You will get an email when someone replies. You can change this under your GitHub profile's notification settings.

### What makes feedback easy to act on

Please include, where you can:

- **Which file** you are commenting on. For example: `README.md` (the specification), `PROCUREMENT-BUYERS-GUIDE.md`.
- **Which section or clause.** For example: *"Section 4, Conformance Testing"* or *"Clause C1.6"*.
- **What you think is wrong, missing or unclear.** One or two sentences is enough.
- **What you would like instead**, if you have a view.
- **Who you are commenting as**, if relevant — for example *"local authority UTC officer"*, *"UTC supplier"*, *"procurement officer"*. This is optional but helps the maintainers weigh the comment.

Raise one topic per issue. Several unrelated points in one issue are hard to track and close.

---

## 4. Comment on a particular line

Use this when you want to point at an exact place in a file.

1. In the repository, click through to the file. The main documents (`README.md`, `PROCUREMENT-BUYERS-GUIDE.md`) are at the top level of the repository: click the file name.
2. The file displays as a formatted page. Click the **Code** button (or the `<>` icon) at the top of the file view to see the underlying text with **line numbers** down the left. (The label varies slightly between GitHub versions: look for a toggle between *Preview* and *Code*.)
3. Click the **line number** of the line you want to comment on. A small `⋯` menu appears beside it.
4. Choose **Reference in new issue**.
5. GitHub opens a new issue with a link to that exact line already filled in. Add a title and your comment, then click **Submit new issue**.

The link to the line stays attached to your comment, so the reader can click straight to the place you meant.

> **Tip:** If you select several lines by clicking the first line number and then holding **Shift** while clicking the last, the issue refers to the whole range.

---

## 5. Reply to an existing issue

If an issue already exists on your topic:

1. Open the **Issues** tab.
2. Click the title of the issue you want to join.
3. Read the existing conversation first.
4. Scroll to the bottom. Type your reply into the **Add a comment** box.
5. Click **Comment**.

You can use the **Preview** tab above the box to see how your text will look before you send it.

### Simple formatting you may find useful

You do not need any of this, and plain text is fine.

| You type | You get |
|---|---|
| `**important**` | **important** |
| `- item` on its own line | a bullet point |
| `> quoted text` | an indented quotation. Useful for quoting the sentence you are replying to |
| `` `C1.6` `` | `C1.6` in a code font. Useful for clause numbers and field names |

---

## 6. Suggest the exact wording (optional)

If you can see precisely how a sentence should read, say so in an issue (section 3). You do not need to change any file yourself.

1. Open a new issue, or reply to an existing one.
2. Quote the current text using a `>` at the start of the line (see the formatting table in section 5).
3. Underneath, write the wording you would like instead, and add a sentence on why.

The maintainers will consider it and either update the document or reply with a question.

**Note:** do not worry about the generated files under `Schemas/`, `Schemas_asyncapi/`, `Diagrams/`, `asyncapi.yaml` or `asyncapi_clients/output/`. These are produced automatically from the models in `topas_3000_model/`. If you think one of them is wrong, open an **issue** (section 3) and say which schema or field. The maintainers will fix the source.

---

## 7. What happens next

1. A maintainer reads your issue.
2. They may reply with a question, so check your email.
3. They will either change the documents, explain why not, or note the point as deferred. The specification's *Deferred and Won't Do* section lists what is outside the current scope.
4. When the matter is settled, they close the issue. A closed issue can still be read, and you can reopen it by replying.

Response times are not guaranteed. Certification is performed by TOPAS, and feedback on this repository does not form part of that process.

---

## 8. Getting help

- GitHub's own beginner guide: <https://docs.github.com/en/get-started/start-your-journey/hello-world>
- How to open an issue: <https://docs.github.com/en/issues/tracking-your-work-with-issues/using-issues/creating-an-issue>
- If you get stuck, open an issue (section 3) saying where you got to and a maintainer will help.

---

## Glossary

| Term | Meaning |
|---|---|
| **Repository** ("repo") | The shared folder that holds all the files for a project, and their history. |
| **Issue** | A titled message used for questions, comments and problem reports. |
| **Maintainer** | A person who looks after the repository and decides what is accepted. |
| **Markdown** (`.md`) | The plain-text format the documents are written in. GitHub displays it as a formatted page. |
