type BridgeMethodCollection = {
  phase: Phase;
};

type Phase = {
  [key: string]: {
    add: (parameter: string, shape?: object, option?: object) => void;
  };
};

type Position = {
  x: number;
  y: number;
};

type Circle = Omit<
  {
    radius: number;
    color: string;
  } & Position,
  never
>;

type Text = Omit<
  {
    val: string;
  } & Position,
  never
>;

type Arrow = {
  from: Position;
  to: Position;
  color: string;
};

type Option = {
  duration: number;
  timingFunc: string;
};

// Public
export function inorderTraverse(collections: BridgeMethodCollection): void {
  const { phase } = collections;
  const WIDTH = 100;
  const tree = [1, 2, 3, 4, null, 5, 6, 7, 8];
  const defaultPosition = {
    x: 600,
    y: 100,
  };

  initPhase(phase);

  function traverse(i: number, depth: number, pos: Position): void {
    const node = tree[i];
    const circle: Circle = {
      radius: 20,
      color: isLeaf(node) ? "red" : "black",
      ...pos,
    };
    const text: Text = {
      val: node ? node.toString() : "",
      ...pos,
    };
    const toLeftPosition = {
      x: pos.x - WIDTH / (depth + 1),
      y: pos.y + 60,
    };
    const toRightPosition = {
      x: pos.x + WIDTH / (depth + 1),
      y: pos.y + 60,
    };
    const toLeftArrow: Arrow = {
      color: "blue",
      from: { ...pos },
      to: toLeftPosition,
    };
    const toRightArrow: Arrow = {
      color: "green",
      from: { ...pos },
      to: toRightPosition,
    };
    const defaultOption = {
      duration: 100,
      timingFunc: "linear",
    };

    // 서브 트리 그리기
    // 리프 노드의 경우 노드만 그린다.
    drawCircle(
      phase,
      circle,
      { ...defaultOption, duration: defaultOption.duration * 3 },
    );
    assertNotLeaf(node);
    drawText(phase, text, defaultOption);
    drawArrow(phase, toLeftArrow, defaultOption);

    // 순회
    traverse(i * 2 + 1, depth + 1, toLeftPosition);
    drawArrow(phase, toRightArrow, defaultOption);
    traverse(i * 2 + 2, depth + 1, toRightPosition);
  }

  traverse(0, 0, defaultPosition);
}

function drawCircle(phase: Phase, shape: Circle, option: Option): void {
  phase["startTraversing"].add(
    "DrawCircle",
    {
      ...shape,
    },
    {
      ...option,
    },
  );
}

function drawText(
  phase: Phase,
  shape: Text,
  option: Option,
): void {
  phase["startTraversing"].add(
    "drawText",
    {
      ...shape,
    },
    {
      ...option,
    },
  );
}

function drawArrow(phase: Phase, shape: Arrow, option: Option) {
  phase["startTraversing"].add(
    "DrawArrow",
    {
      ...shape,
    },
    {
      ...option,
    },
  );
}

function isLeaf(node: number | null | undefined): node is number {
  return node === null || node === undefined;
}

function assertNotLeaf(
  node: number | null | undefined,
): asserts node is number {
  if (isLeaf(node)) {
    return;
  }
}

function initPhase(phase: Phase) {
  phase["color-white-bg"].add("clearBg");
}
