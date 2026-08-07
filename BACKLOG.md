# Backlog

1. Refactor the code by making smaller modules that are easier to maintain

   - [x] Extract the `config` module (API key, AI model, color scheme)
   - [x] Extract the `render` module (answer rendering)
   - [x] Extract the `client` module (OpenRouter API client and spinner logic)
   - [x] Extract the `prompt` module (question input)

2. Create Github actions to generate the release assets

   - [x] Automatically build and publish the `linux_amd64` release asset (`hey_x.x.x_linux_amd64.tar.gz`) on tag push

3. Installation script

   - [x] Add the `install.sh` script that will install the latest release version from the repository.

     The link to the repository `https://github.com/psugrg/hey`.
     The example path to the asset `https://github.com/psugrg/hey/releases/download/v0.1.0/hey_0.1.0_linux_amd64.tar.gz`.
     The example invocation `curl -fsSL https://github.com/psugrg/hey/install.sh | sh`.

   - [x] Extend the installation instructions to contain the step of installing the lates version with `curl`.
   - [x] The installation script should also check if the `OPENROUTER_API_KEY` is available and set. If not,
         the script should guide the user where to register it and how to add it to the environment variable (and to `.bashrc` and `.zshrc`).

4. Create `AGENTS.md` file

   - [x] Initialize the project in `opencode` with `/init` command to create the `AGENTS.md` file. More on that [here](https://opencode.ai/docs/rules/)
   - [x] Cleanup the `README.md` file to not to duplicate information from `AGENTS.md`
   - [x] Add rule to modularize the application. This means that the agent should not keep everything in one file but rather create software modules.
   - [x] Add rule to minimize commets in code. The code should be self-explanatory. Only the inrerfaces should be properly documented.

5. Add a new feature that allows to continue the discussion

   - [x] It should be possible to continue the discussion to add a follow-up questions.
         This should be possible by using the `-f` or `--follow-up` flag.
         It should be possible to continue the last conversation (only). No need to support the full history.

6. Support for `--help` option

   - [x] Implement the help functionality that will be triggered by the `--help` option.

7. Implement unit tests

   - [ ] Implement unit tests for the application.
   - [ ] Add a new entry to the `./AGENTS.md` file that asks to always write unit tests for the new functionality.
   - [ ] Create new GitHub action to run unit tests on each push.

8. Implement support for configuration file

   - [x] Modify the configuration module to accept the configuration file.

     File should be called `hey.toml` and be located in `.config` directory.
     Things that are now hardcoded in the configuration module should stay there and used should be used as defaults.
     The `hey.toml` configuration file should allow to overvrite the defaults.

     List of elements that should be configurable:
     - default model

       The support for model from environment variable `OPENROUTER_MODEL` should be dropped (braking change).

     - default API URL
     - default system prompt

     > [!IMPORTANT]
     > The API key should still be fetched from the environment variable for security reasons.
     > (Some users push their configuration to Github).

   - [x] Update the `Configuration` section of the `./README.md` file by adding the subsection `Configuration file` and explaining all configuration options with examples.
   - [x] Implement the unit tests for the `Configuration` module
   - [x] Implement the integration tests for the `Configuration` module

9. Improve the look-and-feel of the `prompt` and the response from the model.

   - [x] Use extended symbols like `○`, `◇`, `◈`, `●`, `│`, `└` and similar, to create a better prompt layot and response layout.

   Example layout

   ```
   ●
    › What is the speed of light?
   ○
     Approximately 299,792,458 meters per second.
   ◉
   ```

   Propose a better layout if you think it can be improved

   - [x] Change the `spinner` to use the following sequence `◜`, `◝`, `◞`, `◟` "in place", which means that they should look like the animated circle.

   The `○` should be printed in place of the spinner after the answer is generated.

   - [x] Make this function configurable via the `config` module, and via the `hey.toml` file.
   - [x] Rename the following configuration variables (both in code and in the configuration file)

     - `prompt_top_symbol` -> `prompt_open`
     - `prompt_line_symbol` -> `prompt_line`
     - `prompt_done_symbol` -> `prompt_done`
     - `prompt_close_symbol` -> `prompt_close`

     The change is motivated by the fact that the application handles not only one symbol, but it actually handles the whole string (which is even better).

     Don't forget to update the readme section!

10. Implement _buddies_, assistants that can be selected and individually configured

    - [ ] The initial step is to remove the possibility to configure the `model` and `system_prompt` by the `hey.toml` configuration file.
          This will be replaced by the _buddies_ configuration in the next steps.

    - [ ] The `hey.toml` file should allow to configure the assistants called `baddies`.

      _Buddies_ can be selected by providing their name as a command to the application, e.g. `hey John`. The command should be case insensitive.
      Example configuration:

      ```toml
      # the default one can be called without a name
      [[buddies]]
      default = true
      nqqqame = "Tom"
      model = "openai/gpt-4o-mini"
      system_prompt = "your name is Tom and you are a helpful assistant that answers questions about command-line tools and commands (e.g. bash, ls, grep, cat, find, etc). keep answers concise and focused on cli usage."

      [[buddies]]
      name = "John"
      model = "google/gemini-2.5-flash-lit"
      system_prompt = "your name is John and you are a helpful english teacher that can translate words and phrases and explain their meaning to the user who asked the question"
      ```

      If there are more `buddies` marked as `default`, the first one of them should be a default one.

      If the `default` `buddy` is missing, the first one from the configuration file is used as the default one.

      If the configuration file is missing, or if there is no `buddies` configured, the `hey` application should use the hardcoded default one, as it is right now.

    - [ ] Document the configuration the ./README.md` file.

    - [ ] Implement unit tests and integration tests
