# Using Browser Cookies for Authentication

If the website works in your browser but not with the CLI, it's because your browser has authentication cookies that the CLI doesn't have.

## Why This Happens

1. When you first visit datapass.de in your browser, you authenticate (login/verification)
2. The website sets cookies in your browser
3. Your browser automatically sends these cookies with every request
4. The CLI tool doesn't have these cookies, so it sees the "not authenticated" page

## Solution: Extract Cookies from Browser

### Method 1: Using Browser Developer Tools (All Browsers)

#### Firefox

1. Open datapass.de in Firefox
2. Press `F12` to open Developer Tools
3. Go to the **Storage** tab
4. Click on **Cookies** → `https://datapass.de`
5. Look for cookies (common names: `JSESSIONID`, `dtag_session`, etc.)
6. Right-click and "Copy" the cookie value

#### Chrome/Chromium/Edge

1. Open datapass.de in Chrome
2. Press `F12` to open Developer Tools
3. Go to the **Application** tab
4. In the left sidebar: **Storage** → **Cookies** → `https://datapass.de`
5. Copy the cookie name and value

#### Safari

1. Open datapass.de in Safari
2. Develop → Show Web Inspector (enable Develop menu in Preferences first)
3. Go to **Storage** tab
4. Click **Cookies** → `https://datapass.de`
5. Copy the cookie value

### Method 2: Using Browser Extensions

#### Cookie Editor Extensions

- **Firefox**: [Cookie-Editor](https://addons.mozilla.org/en-US/firefox/addon/cookie-editor/)
- **Chrome**: [EditThisCookie](https://chrome.google.com/webstore/detail/editthiscookie/fngmhnnpilhplaeedifhccceomclgfbg)

These extensions let you export all cookies easily.

### Method 3: Get All Cookies at Once

#### In Developer Tools (Network Tab)

1. Open datapass.de in your browser
2. Press `F12` → **Network** tab
3. Reload the page
4. Click on any request to datapass.de
5. Look for **Request Headers** → **Cookie**
6. Copy the entire cookie string

## Using Cookies with datapass CLI

Once you have the cookie string:

```bash
# Simple cookie (single cookie)
./datapass --cookie "JSESSIONID=ABC123XYZ"

# Multiple cookies (semicolon-separated)
./datapass --cookie "JSESSIONID=ABC123XYZ; dtag_session=DEF456"

# Full cookie string from Network tab
./datapass --cookie "JSESSIONID=ABC123; Path=/; dtag_session=XYZ789; other=value"
```

## Complete Example

### Step-by-Step

1. **Open datapass.de in your browser** (make sure it works and shows your data)

2. **Press F12** to open Developer Tools

3. **Go to Network tab** and reload the page

4. **Click on the first request** to datapass.de/home

5. **Find the Cookie header** in Request Headers section

6. **Copy the cookie string** (everything after "Cookie: ")

7. **Use it with the CLI:**
   ```bash
   ./datapass --cookie "JSESSIONID=9D8B7A6C5D4E3F2A1B0C; Path=/"
   ```

## Example Output

```bash
$ ./datapass --cookie "JSESSIONID=..."
Plan: MagentaMobil Prepaid XL
Used:      11.64 GB (23.28%)
Total:     50.00 GB (100%)
Remaining: 38.36 GB (76.72%)
█████▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 23.28%
```

## Security Notes

⚠️ **Important:**

- Cookies contain authentication tokens - treat them like passwords!
- Don't share your cookies with others
- Cookies may expire after some time
- If cookies expire, just get fresh ones from your browser

## Troubleshooting

### Still getting authentication error?

- Make sure you copied the complete cookie string
- Check that the cookies haven't expired (get fresh ones)
- Try copying the full cookie string from the Network tab

### Which cookies do I need?

The most important ones are usually:

- `JSESSIONID` - Java session ID
- `dtag_session` or similar - Telekom session
- Any cookie with "session" or "auth" in the name

When in doubt, copy the entire Cookie header from the Network tab.

## Alternative: Save HTML from Browser

If dealing with cookies is too complex, you can simply save the page from your browser:

1. Open datapass.de in your browser (authenticated)
2. Right-click → "Save Page As" → save as `my-data.html`
3. Run: `./datapass --file my-data.html`

This is easier but requires re-saving the page each time you want updated data.
