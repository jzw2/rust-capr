import { SoundLawComposition, soundlaw_xsampa_to_ipa } from "../pkg/rust_capr";

export class FileArea {
  uploadFile: HTMLInputElement;
  fileStrings: string[];
  tranducer: SoundLawComposition;
  table: HTMLTableRowElement;
  tableHeader: HTMLTableRowElement;
  transduceListner: () => void;

  constructor(transduceListener: () => void, transducer: SoundLawComposition) {
    this.transduceListner = transduceListener;
    this.tranducer = transducer;
    this.table = document.getElementById("file-inputs") as HTMLTableRowElement;
    this.tableHeader = document.getElementById(
      "file-headers",
    ) as HTMLTableRowElement;

    this.uploadFile = document.getElementById("upload") as HTMLInputElement;
    this.fileStrings = [];

    this.uploadFile.addEventListener("change", async () => {
      if (this.uploadFile.files) {
        let file = this.uploadFile.files[0];
        const text = await file.text();

        this.fileStrings = text.split("\n").filter((x) => x !== "");

        this.transduceListner();
        this.renderHeader();
      } else {
        this.fileStrings = [];
      }
    });
  }

  renderHeader() {
    let newChildren = this.fileStrings.map((text) => {
      const elem = document.createElement("th");
      elem.textContent = soundlaw_xsampa_to_ipa(text);
      return elem;
    });
    this.tableHeader.replaceChildren(...newChildren);
  }

  transduce() {
    let body = this.table.querySelector("tbody");
    if (!body) {
      body = document.createElement("tbody");
      this.table.appendChild(body);
    }

    const transduced = this.fileStrings.map((s) =>
      this.tranducer.transduce_text(s),
    );

    if (transduced[0]) {
      const transpose = transduced[0].map((_, index) =>
        transduced.map((row) => {
          if (!row[index]) {
            return "";
          }
          return soundlaw_xsampa_to_ipa(row[index].replaceAll(" ", ""));
        }),
      );

      const newChildren = transpose.map((r) => {
        const row = document.createElement("tr");
        const rowElements = r.map((d) => {
          const data = document.createElement("td");
          data.innerText = soundlaw_xsampa_to_ipa(d);
          return data;
        });
        row.append(...rowElements);
        return row;
      });
      body.replaceChildren(...newChildren);
    } else {
      body.replaceChildren();
    }
  }
}
