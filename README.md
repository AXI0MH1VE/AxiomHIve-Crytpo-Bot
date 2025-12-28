# AxiomHIve-Crypto-Bot

AxiomHIve-Crypto-Bot is a powerful and user-friendly cryptocurrency bot designed to automate trading and portfolio management in the fast-paced world of cryptocurrency markets.

## Setup

1. **Clone the Repository**
   ```bash
   git clone https://github.com/AXI0MH1VE/AxiomHIve-Crytpo-Bot.git
   cd AxiomHIve-Crytpo-Bot
   ```

2. **Install Dependencies**
   Make sure `pip` and Python (>= 3.7) are installed.
   ```bash
   pip install -r requirements.txt
   ```

3. **Set up Environment Variables**
   Create a `.env` file in the root directory with the following variables:
   ```env
   API_KEY=your_api_key
   API_SECRET=your_api_secret
   ```

4. **Run Migrations** (if required)
   ```bash
   python manage.py migrate
   ```

## Usage

1. **Start the Bot**
   ```bash
   python bot.py
   ```

2. **Monitor Logs**
   The bot logs its activities in `logs/bot.log`, which allows you to monitor its behavior and troubleshoot as needed.

3. **Stop the Bot**
   You can stop the bot with `CTRL+C` if you are running it in a terminal.

## Example Commands

Here are some example commands supported by the bot:

1. **Fetch Current Portfolio**
   ```plaintext
   portfolio
   ```
   Returns:
   ```plaintext
   [BTC: 0.1, ETH: 2.5, ADA: 100]
   ```

2. **Execute Trade**
   ```plaintext
   trade BTC/USDT buy 0.01
   ```
   Executes a trade for 0.01 BTC bought in exchange for USDT.

3. **Get Market Overview**
   ```plaintext
   market_overview
   ```
   Provides:
   ```plaintext
   BTC: $40000, ETH: $3000, ADA: $1.5
   ```

Feel free to explore the other commands by using the `help` command within the bot interface.

---

Contributions are welcome! Simply fork the repository, create a new branch with your changes, and open a pull request.