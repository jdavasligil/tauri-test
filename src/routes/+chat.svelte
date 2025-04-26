<script lang="ts">
  // import { invoke } from "@tauri-apps/api/core";

  // let name = $state("");
  // let greetMsg = $state("");
  //
  // async function greet(event: Event) {
  //   event.preventDefault();
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   greetMsg = await invoke("greet", { name });
  // }
</script>

<main class="container">
  <div class="header">
    <img src="/logo.jpg" alt="Logo" id="logo" class="login-btn" />
    <div class="buttons">
      <!-- Twitch Login Button -->
      <button
        id="twitchLoginButton"
        title="Login with Twitch"
        class="login-btn"
      >
        <img src="/twitch-tile.svg" alt="Login with Twitch" />
      </button>

      <!-- Logout Button -->
      <button id="logoutButton" style="display: none">logout</button>
      <!-- Refresh and Popout Buttons -->
      <button id="refreshServerBtn" title="Refresh Server">
        <img src="refresh2.png" alt="Refresh" />
      </button>
      <button id="popoutChatBtn" title="Popout Chat">
        <img src="popout.png" alt="Popout" />
      </button>
    </div>
  </div>

  <div id="chat-container"></div>

  <div id="message-send-container">
    <input
      type="text"
      id="messageInput"
      class="message-input-box"
      placeholder="Type a message..."
    />
    <button id="sendMessageButton" class="send-btn">send</button>
  </div>

  <!-- <h1>Welcome to Tauri + Svelte</h1> -->
  <!---->
  <!-- <div class="row"> -->
  <!--   <a href="https://vitejs.dev" target="_blank"> -->
  <!--     <img src="/vite.svg" class="logo vite" alt="Vite Logo" /> -->
  <!--   </a> -->
  <!--   <a href="https://tauri.app" target="_blank"> -->
  <!--     <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" /> -->
  <!--   </a> -->
  <!--   <a href="https://kit.svelte.dev" target="_blank"> -->
  <!--     <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" /> -->
  <!--   </a> -->
  <!-- </div> -->
  <!-- <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p> -->
  <!---->
  <!-- <form class="row" onsubmit={greet}> -->
  <!--   <input id="greet-input" placeholder="Enter a name..." bind:value={name} /> -->
  <!--   <button type="submit">Greet</button> -->
  <!-- </form> -->
  <!-- <p>{greetMsg}</p> -->
</main>

<!-- <style> -->
<!--   .logo.vite:hover { -->
<!--     filter: drop-shadow(0 0 2em #747bff); -->
<!--   } -->
<!---->
<!--   .logo.svelte-kit:hover { -->
<!--     filter: drop-shadow(0 0 2em #ff3e00); -->
<!--   } -->
<!---->
<!--   :root { -->
<!--     font-family: Inter, Avenir, Helvetica, Arial, sans-serif; -->
<!--     font-size: 16px; -->
<!--     line-height: 24px; -->
<!--     font-weight: 400; -->
<!---->
<!--     color: #0f0f0f; -->
<!--     background-color: #f6f6f6; -->
<!---->
<!--     font-synthesis: none; -->
<!--     text-rendering: optimizeLegibility; -->
<!--     -webkit-font-smoothing: antialiased; -->
<!--     -moz-osx-font-smoothing: grayscale; -->
<!--     -webkit-text-size-adjust: 100%; -->
<!--   } -->
<!---->
<!--   .container { -->
<!--     margin: 0; -->
<!--     padding-top: 10vh; -->
<!--     display: flex; -->
<!--     flex-direction: column; -->
<!--     justify-content: center; -->
<!--     text-align: center; -->
<!--   } -->
<!---->
<!--   .logo { -->
<!--     height: 6em; -->
<!--     padding: 1.5em; -->
<!--     will-change: filter; -->
<!--     transition: 0.75s; -->
<!--   } -->
<!---->
<!--   .logo.tauri:hover { -->
<!--     filter: drop-shadow(0 0 2em #24c8db); -->
<!--   } -->
<!---->
<!--   .row { -->
<!--     display: flex; -->
<!--     justify-content: center; -->
<!--   } -->
<!---->
<!--   a { -->
<!--     font-weight: 500; -->
<!--     color: #646cff; -->
<!--     text-decoration: inherit; -->
<!--   } -->
<!---->
<!--   a:hover { -->
<!--     color: #535bf2; -->
<!--   } -->
<!---->
<!--   h1 { -->
<!--     text-align: center; -->
<!--   } -->
<!---->
<!--   input, -->
<!--   button { -->
<!--     border-radius: 8px; -->
<!--     border: 1px solid transparent; -->
<!--     padding: 0.6em 1.2em; -->
<!--     font-size: 1em; -->
<!--     font-weight: 500; -->
<!--     font-family: inherit; -->
<!--     color: #0f0f0f; -->
<!--     background-color: #ffffff; -->
<!--     transition: border-color 0.25s; -->
<!--     box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2); -->
<!--   } -->
<!---->
<!--   button { -->
<!--     cursor: pointer; -->
<!--   } -->
<!---->
<!--   button:hover { -->
<!--     border-color: #396cd8; -->
<!--   } -->
<!--   button:active { -->
<!--     border-color: #396cd8; -->
<!--     background-color: #e8e8e8; -->
<!--   } -->
<!---->
<!--   input, -->
<!--   button { -->
<!--     outline: none; -->
<!--   } -->
<!---->
<!--   #greet-input { -->
<!--     margin-right: 5px; -->
<!--   } -->
<!---->
<!--   @media (prefers-color-scheme: dark) { -->
<!--     :root { -->
<!--       color: #f6f6f6; -->
<!--       background-color: #2f2f2f; -->
<!--     } -->
<!---->
<!--     a:hover { -->
<!--       color: #24c8db; -->
<!--     } -->
<!---->
<!--     input, -->
<!--     button { -->
<!--       color: #ffffff; -->
<!--       background-color: #0f0f0f98; -->
<!--     } -->
<!--     button:active { -->
<!--       background-color: #0f0f0f69; -->
<!--     } -->
<!--   } -->
<!-- </style> -->
<style>
  :global(body),
  :global(html) {
    margin: 0;
    padding: 0;
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: rgba(0, 0, 0, 0);
    color: white;
    font-family: Arial, sans-serif;
    forced-color-adjust: none;
  }

  .header {
    padding: 0 10px; /* Horizontal padding of 10px */
    padding-top: 5px;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  #chat-container {
    display: flex;
    flex-direction: column-reverse;
    flex: 1;
    overflow-y: auto;
    padding: 0 10px; /* Horizontal padding to match header */
  }

  #chat-container::-webkit-scrollbar {
    display: none;
  }

  #chat-container {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }

  #logo {
    height: 30px;
    width: 30px;
    border-radius: 50%;
    margin-right: 10px; /* Adjust margin to 10px */
  }

  .chat-message {
    margin: 4px 0;
    opacity: 0;
    animation: glideInBounce 0.5s forwards;
    overflow-wrap: break-word;
    word-wrap: break-word;
  }

  .badge-icon {
    height: 18px;
    width: 18px;
    margin-right: 5px;
    vertical-align: middle;
  }

  .emote-img {
    height: 28px;
    width: 28px;
    vertical-align: middle;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .buttons {
    display: flex;
    align-items: right;
    justify-content: space-between;
    gap: 10px;
  }

  .login-btn img,
  #popoutChatBtn img,
  #refreshServerBtn img {
    height: 30px;
    width: 30px;
  }

  .login-btn,
  #popoutChatBtn,
  #refreshServerBtn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
  }

  @keyframes glideInBounce {
    0% {
      transform: translateX(100%);
      opacity: 0;
    }
    70% {
      transform: translateX(-10%);
      opacity: 1;
    }
    100% {
      transform: translateX(0);
      opacity: 1;
    }
  }

  #message-send-container {
    display: flex;
    align-items: center;
    margin: 10px 5px 0 5px; /* Consistent margin on all sides */
    padding-bottom: 10px;
  }

  .message-input-box {
    flex-grow: 1;
    width: auto;
    padding: 0 20px;
    height: 50px;
    border: 1px solid #666;
    background: #333;
    color: white;
    font-size: 1.2em;
    border-radius: 5px;
    margin-right: 10px; /* Adjust margin to 10px */
  }

  .send-btn {
    flex-shrink: 0; /* Prevent the send button from shrinking */
    padding: 10px 20px;
    height: 50px;
    border: 1px solid #666;
    background-color: #444;
    color: #bbb;
    font-size: 1em;
    cursor: pointer;
    border-radius: 5px;
    transition: background-color 0.3s;
  }

  #logoutButton {
    background: none; /* No background */
    border: none; /* No border */
    color: #bbb; /* Light grey color, adjust if needed */
    cursor: pointer; /* Cursor changes to pointer to indicate it's clickable */
    padding: 0; /* No padding */
    font-size: 1em; /* Adjust the size as needed */
    transition: color 0.3s; /* Transition for color change on hover */
  }

  .message-input-box:focus {
    outline: none;
  }

  .send-btn:hover {
    background-color: #555;
  }

  * {
    box-sizing: border-box;
  }

  input[type="text"],
  button {
    padding: 10px;
  }

  .header,
  #message-send-container {
    display: flex;
    flex-shrink: 1; /* Allow shrinkage when necessary */
  }

  /**
 * Message effects
 */
  .text-bold {
    font-weight: bold;
  }

  .text-italic {
    font-style: italic;
  }

  .color-yellow {
    color: #ffff00;
  }

  .color-red {
    color: #ff0000;
  }

  .color-green {
    color: #00ff00;
  }

  .color-cyan {
    color: #00ffff;
  }

  .color-purple {
    color: #9c59d1;
  }

  .color-pink {
    color: #ff00ff;
  }

  .color-rainbow {
    background: linear-gradient(
      to right,
      #ef5350,
      #f48fb1,
      #7e57c2,
      #2196f3,
      #26c6da,
      #43a047,
      #eeff41,
      #f9a825,
      #ff5722
    );

    background-clip: text;
    -webkit-background-clip: text;

    -webkit-text-fill-color: transparent;
  }

  @keyframes flash1 {
    0% {
      color: #ff0000;
    }
    50% {
      color: #ffff00;
    }
  }

  @keyframes flash2 {
    0% {
      color: #0000ff;
    }
    50% {
      color: #00ffff;
    }
  }

  @keyframes flash3 {
    0% {
      color: #00b000;
    }
    50% {
      color: #80ff80;
    }
  }

  .color-flash1 {
    animation: flash1 0.45s steps(1, end) infinite;
  }

  .color-flash2 {
    animation: flash2 0.45s steps(1, end) infinite;
  }

  .color-flash3 {
    animation: flash3 0.45s steps(1, end) infinite;
  }

  @keyframes glow1 {
    0% {
      color: #ff0000;
    }
    33% {
      color: #00b000;
    }
    66% {
      color: #0000ff;
    }
    100% {
      color: #ff0000;
    }
  }

  @keyframes glow2 {
    0% {
      color: #ff0000;
    }
    33% {
      color: #800080;
    }
    66% {
      color: #0000ff;
    }
    100% {
      color: #ff0000;
    }
  }

  @keyframes glow3 {
    0% {
      color: #ffffff;
    }
    25% {
      color: #00b000;
    }
    50% {
      color: #ffffff;
    }
    67.5% {
      color: #00ffff;
    }
    75% {
      color: #0000ff;
    }
    100% {
      color: #ffffff;
    }
  }

  .color-glow1 {
    animation: glow1 3s linear infinite;
  }

  .color-glow2 {
    animation: glow2 3s linear infinite;
  }

  .color-glow3 {
    animation: glow2 3s linear infinite;
  }

  .effect-wave {
    /* TODO: implement this */
    animation: glow1 3s linear infinite;
  }

  .effect-shake {
    /* TODO: implement this */
    animation: glow1 3s linear infinite;
  }
</style>
