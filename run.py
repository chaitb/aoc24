from sys import argv
import markdownify
import requests
import os
from openai import OpenAI
from dotenv import load_dotenv

load_dotenv()

session_cookie = os.getenv("AOC_SESSION")
openai_key = os.getenv("OPENAI_API_KEY")

if (not session_cookie or not openai_key):
    print("Please set the environment variables AOC_SESSION and OPENAI_API_KEY")
    exit(1)

client = OpenAI(
    api_key=openai_key,
)

prompts = {
    "strip": "Trim the unnecessary stuff from the following markdown, and give me a clean version of the problem statement",
    "intro": "Look at the following problem, and provide a short explanation, followed by a summary of the approach. Then provide the pseudocode (plain language) for the solution",
    "code": "Now based on the approach, give me just executable rust code (main.rs, not in md) for this solution. No extra text - put any explanation in comments only if necessary. The input is provided via stdin, loop over the input lines using `for line in read_lines() { ... }`"
}

def ai_strip(inp: str, model="gpt-4o-mini"):
    chat_completion = client.chat.completions.create(
        messages=[
            {
                "role": "user",
                "content": prompts["strip"] + "\n" + inp,
            }
        ],
        model=model,
    )
    return chat_completion.choices[0].message.content

def ai_analyse(inp: str, model="o1-preview"):
    chat_completion = client.chat.completions.create(
        messages=[
            {
                "role": "user",
                "content": prompts["intro"] + "\n" + inp,
            }
        ],
        model=model,
    )
    return chat_completion.choices[0].message.content

def ai_code(inp: str, analysis: str, model="o1-preview"):
    chat_completion = client.chat.completions.create(
        messages=[
            {
                "role": "user",
                "content": prompts["intro"] + "\n" + inp,
            },
            {
                "role": "assistant",
                "content": analysis,
            },
            {
                "role": "user",
                "content": prompts["code"],
            }
        ],
        model=model,
    )
    return chat_completion.choices[0].message.content


def save_markdown(content: str, path: str):
    with open(path, "w") as file:
        file.write(content)

def make_request(url):
    session = requests.Session()
    response = session.get(url, cookies={ "session": session_cookie or "" })
    response.raise_for_status()
    return response

def process(inputs, day: int, year:int=2024):
    # create folder
    os.makedirs(folder, exist_ok=True)

    print(f"Fetching data for day {day} of year {year}")
    res = make_request(inputs["question"] + "#part2") # part 2, just in case
    res.raise_for_status()

    md_content = markdownify.markdownify(res.text)

    print("Cleaning up the problem statement...")
    trimmed = ai_strip(md_content) or md_content

    save_markdown(trimmed, inputs["questionFile"])

    print("Fetching input data...")
    res = make_request(inputs["input"])
    res.raise_for_status()

    with open(inputs["inputFile"], "w") as file:
        file.write(res.text)

    print("Analysing the problem...")
    analysis = ai_analyse(trimmed, "o1-preview") or "## No analysis available"
    save_markdown(analysis, inputs["analysisFile"])

    print("Generating code...")
    code = ai_code(trimmed, analysis) or "fn main() { println!(\"No code available\") }"

    with open(inputs["codeFile"], "w") as file:
        file.write(code)

    return True

# args: day, year
if __name__ == "__main__":
    day = int(argv[1]) if len(argv) > 1 else 1
    year = int(argv[2]) if len(argv) > 2 else 2024

    folder = argv[3] if len(argv) > 3 else f"src/bin/{day}-ai"

    data = {
        "question" : f"https://adventofcode.com/{year}/day/{day}",
        "questionFile" : f"{folder}/question.md",
        "input" : f"https://adventofcode.com/{year}/day/{day}/input",
        "inputFile" : f"{folder}/input.txt",
        "analysisFile" : f"{folder}/analysis.md",
        "codeFile" : f"{folder}/main.rs"
    }

    res = process(data, day, year)
    if res:
        print("All done!")
    else:
        print("Something went wrong")

    # pretty print the data
    for key, value in data.items():
        print(f"{key}: {value}")
