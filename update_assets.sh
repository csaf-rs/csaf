#!/usr/bin/env bash

rsync -c csaf/csaf_2.0/json_schema/csaf_json_schema.json csaf-rs/assets/csaf_2.0_json_schema.json
rsync -c csaf/csaf_2.1/json_schema/csaf.json csaf-rs/assets/csaf_2.1_json_schema.json
rsync -c ssvc/data/schema/v1/Decision_Point-1-0-1.schema.json csaf-rs/assets/decision_point_1.0.1_json_schema.json
rsync -cr --delete ssvc/data/json/decision_points/ csaf-rs/assets/ssvc_decision_points/
