import { sendMessage } from "..";
import { SoundClassName, RegexType, State } from "./types";
import { soundlaw_xsampa_to_ipa } from "../pkg/rust_capr";

// Since some things are only set once
// putting it in render seems to be kind of wasteful
// this function is only called once
export const renderInit = () => {
  const uploadFile = document.getElementById("upload") as HTMLInputElement;

  uploadFile.addEventListener("change", async () => {
    if (uploadFile.files) {
      let file = uploadFile.files[0];
      const text = await file.text();
      sendMessage({ type: "UploadFile", contents: text });
    } else {
      sendMessage({ type: "UploadFile", contents: "" });
    }
  });

  const left = document.getElementById("left-input") as HTMLInputElement;
  const right = document.getElementById("right-input") as HTMLInputElement;
  const to = document.getElementById("to") as HTMLInputElement;
  const from = document.getElementById("from") as HTMLInputElement;

  const createButton = document.getElementById(
    "create-law",
  ) as HTMLButtonElement;

  createButton.addEventListener("click", () => {
    const leftSelect = document.querySelector(
      "#left-select",
    ) as HTMLInputElement;
    const rightSelect = document.querySelector(
      "#right-select",
    ) as HTMLInputElement;
    let l: string | SoundClassName = left.value;
    let r: string | SoundClassName = right.value;
    if (leftSelect.value !== "") {
      l = { name: leftSelect.value };
    }
    if (rightSelect.value !== "") {
      r = { name: rightSelect.value };
    }
    sendMessage({
      type: "AddSoundLaw",
      law: {
        left: l,
        right: r,
        from: from.value,
        to: to.value,
      },
    });
  });

  const input = document.getElementById("input") as HTMLInputElement;
  const backwards = document.getElementById("backward") as HTMLInputElement;
  input?.addEventListener("input", () =>
    sendMessage({ type: "ChangeInput", input: input.value }),
  );
  backwards?.addEventListener("input", () =>
    sendMessage({ type: "ChangeBackwardsInput", input: backwards.value }),
  );

  const saveButton = document.getElementById("save") as HTMLButtonElement;
  const loadButton = document.getElementById("load") as HTMLButtonElement;
  saveButton.addEventListener("click", () => {
    sendMessage({ type: "Save" });
  });
  loadButton.addEventListener("click", () => {
    sendMessage({ type: "Load" });
  });

  const regexSelect = document.getElementById("regex") as HTMLSelectElement;
  regexSelect.addEventListener("input", (e) => {
    sendMessage({
      type: "ChangeRegexType",
      regex: { type: regexSelect.value } as RegexType,
    });
  });

  const createSoundLine = document.querySelector(
    ".create-sound-class",
  ) as HTMLButtonElement;
  createSoundLine.addEventListener("click", () => {
    const inputPhonemes = document.querySelector(
      ".phonemes-input",
    ) as HTMLInputElement;
    const namePhonemes = document.querySelector(
      ".phonemes-name",
    ) as HTMLInputElement;
    if (inputPhonemes && namePhonemes) {
      sendMessage({
        type: "AddSoundClass",
        regex: { type: "Disjunction" },
        name: namePhonemes.value,
        sounds: inputPhonemes.value.split(" "),
      });
    }

    const inputConcat = document.querySelector(
      ".concat-input",
    ) as HTMLInputElement;
    const nameConcat = document.querySelector(
      ".concat-name",
    ) as HTMLInputElement;
    if (inputConcat && nameConcat) {
      sendMessage({
        type: "AddSoundClass",
        regex: { type: "Concat" },
        name: namePhonemes.value,
        sounds: inputPhonemes.value.split(" "),
      });
    }
  });
};
export const render = (state: State) => {
  console.log("Starting rendering with state ", state);
  const loading = document.getElementById("loading");
  if (loading) {
    if (state.isLoading) {
      loading.style.display = "block";
    } else {
      loading.style.display = "none";
    }
    // loading.style.display = "block";
  }
  const output = document.getElementById("output") as HTMLParagraphElement;
  output.innerHTML = state.output
    .map((x) => soundlaw_xsampa_to_ipa(x.replaceAll(" ", "")))
    .join("\n");
  const backwardsOutput = document.getElementById(
    "backwards-output",
  ) as HTMLParagraphElement;
  backwardsOutput.innerHTML = state.revereseOutput
    .map((x) => soundlaw_xsampa_to_ipa(x.replaceAll(" ", "")))
    .join("\n");
  const rulesList = document.getElementById(
    "rulesList",
  ) as HTMLParagraphElement;

  rulesList.innerHTML = "";
  state.soundLawInputs.forEach((x, index) => {
    let left = "";
    if (typeof x.left == "string") {
      left = x.left;
    } else {
      left = x.left.name;
    }
    let right = "";
    if (typeof x.right == "string") {
      right = x.right;
    } else {
      right = x.right.name;
    }
    let s = `Rule: ${x.from} → ${x.to} / ${left} _ ${right}`;
    const listItem = document.createElement("li");
    listItem.textContent = s;
    listItem.draggable = true;

    const deleteButton = document.createElement("button");
    deleteButton.textContent = "Delete";
    deleteButton.classList.add("delete-button");
    deleteButton.addEventListener("click", () => {
      sendMessage({ type: "ClickDelete", index: index });
    });
    const list = document.getElementById("rulesList") as HTMLUListElement;

    const input = document.createElement("input");
    const button = document.createElement("button");
    button.innerHTML = "Click to Move Index";

    button.addEventListener("click", () => {
      const newIndex = parseInt(input.value, 10) - 1; // because lists are 1 based

      if (!isNaN(newIndex)) {
        sendMessage({ type: "Rearrange", old: index, new: newIndex });
      } else {
        console.log("Invalid index input");
      }
    });

    listItem.appendChild(deleteButton);
    listItem.appendChild(input);
    listItem.appendChild(button);
    rulesList.appendChild(listItem);
  });

  if (state.drag.type == "DraggingOver") {
    rulesList.insertBefore(
      rulesList.children[state.drag.old],
      rulesList.children[state.drag.new],
    );
  }
  let table = document.getElementById("file-inputs") as HTMLTableRowElement;
  table.innerHTML = ' <thead> <tr id="file-headers"></tr> </thead> ';
  let tableHeader = document.getElementById(
    "file-headers",
  ) as HTMLTableRowElement;

  state.fileStrings.forEach((line) => {
    const item = document.createElement("th");
    item.textContent = soundlaw_xsampa_to_ipa(line);
    tableHeader.appendChild(item);
  });

  if (state.transducedFileStrings.length > 0) {
    const transpose = state.transducedFileStrings[0].map((_, index) =>
      state.transducedFileStrings.map((row) =>
        soundlaw_xsampa_to_ipa(row[index].replaceAll(" ", "")),
      ),
    );

    transpose.forEach((row) => {
      const htmlRow = document.createElement("tr");
      row.forEach((col) => {
        const item = document.createElement("td");
        item.textContent = col;
        htmlRow.appendChild(item);
      });
      table.append(htmlRow);
    });
  }

  // sound class
  const phonemes = document.querySelector(".phonemes-name");
  if (state.regexType.type == "Disjunction" && phonemes == null) {
    const area = document.querySelector(".sound-class-area");
    const nameDiv = document.createElement("div");
    const nameHeader = document.createElement("p");
    nameHeader.innerHTML = "Name";
    nameDiv.append(nameHeader);
    const name = document.createElement("input");
    name.classList.add("phonemes-name");
    nameDiv.append(name);
    area?.append(nameDiv);

    const instructions = document.createElement("p");
    instructions.innerHTML = "Put the phonemes with one space between each one";
    const phonemes = document.createElement("input");
    phonemes.classList.add("phonemes-input");
    area?.append(instructions);

    area?.append(phonemes);
  } else if (
    state.regexType.type == "Concat" &&
    document.querySelector(".concat-name") == null
  ) {
    const area = document.querySelector(".sound-class-area");
    const nameDiv = document.createElement("div");
    const nameHeader = document.createElement("p");
    nameHeader.innerHTML = "Name";
    nameDiv.append(nameHeader);
    const name = document.createElement("input");
    name.classList.add("concat-name");
    nameDiv.append(name);
    area?.append(nameDiv);

    const instructions = document.createElement("p");
    instructions.innerHTML = "Put the sound classes with one space in between";
    const phonemes = document.createElement("input");
    phonemes.classList.add("concat-input");
    area?.append(instructions);

    area?.append(phonemes);
  }
  // show sound rules
  const soundList = document.querySelector("#sound-classes");
  if (soundList) {
    soundList.innerHTML = "";
    state.soundClasses.forEach((sound) => {
      const li = document.createElement("li");
      li.innerHTML = `${sound.name}: ${sound.sounds.join(",")}`;
      soundList.append(li);
    });
  }

  const leftSelect = document.querySelector("#left-select") as HTMLInputElement;
  const rightSelect = document.querySelector(
    "#right-select",
  ) as HTMLInputElement;
  leftSelect.innerHTML = "<option value=''> Custom </option> ";
  rightSelect.innerHTML = "<option value=''> Custom </option> ";

  state.soundClasses.forEach((law) => {
    const leftNode = document.createElement("option");
    leftNode.innerHTML = law.name;
    leftNode.value = law.name;
    const rightNode = document.createElement("option");
    rightNode.innerHTML = law.name;
    rightNode.value = law.name;

    leftSelect.append(leftNode);
    rightSelect.append(rightNode);
  });

  console.log("Finished rendering");
};
