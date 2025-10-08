import csv
import json

def election_csv_to_json(file_name):
    election = {
        "id": file_name,
        "regions": {}
    }

    election_dict = {}

    with open(f"{file_name}.csv", mode='r') as file:
        csv_reader = csv.reader(file)
        headers = next(csv_reader)
        for row in csv_reader:
            key = row[0]
            election_dict[key] = {headers[i]: row[i] for i in range(1, len(headers))}

    for state, results in election_dict.items():
        state_results = {}
        for candidate, votes in results.items():
            state_results[candidate] = int(votes.replace(',', ''))
        election["regions"][state] = state_results

    json.dump(election, open(f"{file_name}.json", "w"))
