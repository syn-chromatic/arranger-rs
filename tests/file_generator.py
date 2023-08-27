import os


class FileGeneratorBase:
    def __init__(self, dir_path: str):
        self._dir_path = dir_path
        self._max_number = None
        self._ensure_dir_path()

    def _ensure_dir_path(self):
        if not os.path.exists(self._dir_path):
            os.makedirs(self._dir_path)

    def _get_files(self) -> list[str]:
        files = []

        for file in os.listdir(self._dir_path):
            if file.startswith("file_") and file.endswith(".txt"):
                files.append(file)
        return files

    def _get_file_numbers(self, files: list[str]) -> list[int]:
        numbers = []

        for file in files:
            number_suffix = file.split("_")[1]
            number = int(number_suffix.split(".")[0])
            numbers.append(number)
        return numbers

    def _get_next_file_number(self) -> int:
        if self._max_number:
            self._max_number += 1
            return self._max_number

        files = self._get_files()
        numbers = self._get_file_numbers(files)
        if numbers:
            max_number = max(numbers)
            self._max_number = max_number
            return max_number
        self._max_number = 1
        return 1

    def _get_max_file_number(self) -> int:
        files = self._get_files()
        numbers = self._get_file_numbers(files)
        if numbers:
            max_number = max(numbers)
            return max_number
        return 1

    def _update_max_file_number(self) -> int:
        if self._max_number is not None:
            self._max_number += 1
            return self._max_number

        max_number = self._get_max_file_number()
        self._max_number = max_number
        return self._max_number

    def _get_next_file_path(self, max_number: int) -> str:
        filename = f"file_{max_number + 1}.txt"
        path = os.path.join(self._dir_path, filename)
        return path

    def _print_increment_progress(self, max_number: int, increment: int):
        if max_number % increment == 0:
            print(f"Generated File #{self._max_number}", end="\r")

    def _print_progress(self):
        print(f"Generated File #{self._max_number}", end="\r")


class FileGenerator(FileGeneratorBase):
    def __init__(self, dir_path="./"):
        super().__init__(dir_path)

    def generate_empty_files(self, number: int):
        max_number = 0
        while True:
            max_number = self._update_max_file_number()
            if max_number >= number:
                break

            next_file_path = self._get_next_file_path(max_number)
            with open(next_file_path, "w") as _:
                pass
            self._print_increment_progress(max_number, 50)

        self._print_progress()


if __name__ == "__main__":
    fg = FileGenerator("./generated_files")
    fg.generate_empty_files(500_000)
