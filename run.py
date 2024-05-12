from playwright.sync_api import sync_playwright


def get_string():
    program = ""
    while True:
        try:
            line = input()
            program += line + "\n"
        except EOFError:
            break

    return program


def main():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=False)
        page = browser.new_page()
        page.goto("http://16.170.124.96:3001/")
        program = get_string()
        print("Filling program text area")
        element = page.query_selector("#pad_program")
        element.fill(program)
        print("Clicking run button")
        element = page.query_selector("#RunBtn")
        element.click()
        print("Press ctrl+c to exit.")
        while True:
            pass


if __name__ == "__main__":
    main()
