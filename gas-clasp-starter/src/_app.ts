/* eslint-disable @typescript-eslint/no-unused-vars */

function onOpen() {
  try {
    console.log('Hello World!');
  } catch (error: unknown) {
    if (error instanceof Error) {
      showError(error);
    }
  }
}

const showError = (error: Error) => {
    const ui = SpreadsheetApp.getUi();
    const title = "エラーが発生しました。";
    ui.alert(title, error.message, ui.ButtonSet.YES_NO);
  };